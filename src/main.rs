mod models;

use clap::{Parser, Subcommand};
use models::{Config, MealPlan, Meal, MealType, Day};
use std::path::PathBuf;
use chrono::{NaiveDate, Weekday, Local};
use std::io::{self, Write};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Optional custom path for config and data files
    #[arg(short, long, global = true)]
    path: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Add a new meal to the plan
    Add {
        /// Description of the meal
        description: String,
        
        #[arg(short = 't', long)]
        meal_type: String,
        #[arg(short, long)]
        day: String,
        #[arg(short, long)]
        cook: String,
    },
    /// Edit an existing meal in the plan
    Edit {
        /// New description for the meal (optional)
        description: Option<String>,
        
        #[arg(short = 't', long)]
        meal_type: String,
        #[arg(short, long)]
        day: String,
        #[arg(short, long)]
        cook: Option<String>,
    },
    /// Remove a meal from the plan
    Remove {
        #[arg(short, long)]
        meal_type: String,
        #[arg(short, long)]
        day: String,
    },
    /// Export the meal plan to iCal format
    ExportIcal {
        #[arg(short, long)]
        output: PathBuf,
    },
    /// Export the meal plan to JSON format
    ExportJson {
        #[arg(short, long)]
        output: PathBuf,
    },
    /// Sync the meal plan with a remote source
    Sync {
        #[arg(short, long)]
        source: String,
    },
    /// Initialize or update the configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Subcommand, Debug)]
enum ConfigAction {
    /// Initialize the configuration
    Init,
}

fn main() {
    let args = Args::parse();

    // Initialize default configuration
    let config = Config::new();
    let meal_plan_path = config.meal_plan_storage_path.join("meal_plan.json");

    // Load or create a new meal plan
    let mut meal_plan = MealPlan::load_from_json(&meal_plan_path).unwrap_or_else(|_| {
        println!("No existing meal plan found. Creating a new one.");
        MealPlan::new(Local::now().date_naive())
    });

    match args.command {
        Some(Commands::Add { description, meal_type, day, cook}) => {
            match add_meal(&mut meal_plan, meal_type, day, cook, description) {
                Ok(_) => {
                    println!("Meal added successfully.");
                    // Save the updated meal plan
                    if let Err(e) = meal_plan.save_to_json(&meal_plan_path) {
                        eprintln!("Failed to save meal plan: {}", e);
                    }
                }
                Err(e) => eprintln!("Failed to add meal: {}", e),
            }
        }
        Some(Commands::Edit { description, meal_type, day, cook }) => {
            match edit_meal(&mut meal_plan, meal_type, day, cook, description) {
                Ok(_) => {
                    println!("Meal updated successfully.");
                    // Save the updated meal plan
                    if let Err(e) = meal_plan.save_to_json(&meal_plan_path) {
                        eprintln!("Failed to save meal plan: {}", e);
                    }
                }
                Err(e) => eprintln!("Failed to edit meal: {}", e),
            }
        }
        Some(Commands::Remove { meal_type, day }) => {
            match remove_meal(&mut meal_plan, meal_type, day) {
                Ok(_) => {
                    println!("Meal removed successfully.");
                    // Save the updated meal plan
                    if let Err(e) = meal_plan.save_to_json(&meal_plan_path) {
                        eprintln!("Failed to save meal plan: {}", e);
                    }
                }
                Err(e) => eprintln!("Failed to remove meal: {}", e),
            }
        }
        Some(Commands::ExportIcal { output }) => {
            match export_ical(&meal_plan, &output) {
                Ok(_) => println!("Meal plan exported to iCal successfully: {:?}", output),
                Err(e) => eprintln!("Failed to export meal plan to iCal: {}", e),
            }
        }
        Some(Commands::ExportJson { output }) => {
            match export_json(&meal_plan, &output) {
                Ok(_) => println!("Meal plan exported to JSON successfully: {:?}", output),
                Err(e) => eprintln!("Failed to export meal plan to JSON: {}", e),
            }
        }
        Some(Commands::Sync { source }) => {
            println!("Syncing meal plan with: {}", source);
            // TODO: Implement sync function
        }
        Some(Commands::Config { action: ConfigAction::Init }) => {
            println!("Initializing configuration");
            // TODO: Implement config_init function
        }
        None => {
            println!("Welcome to the Meal Plan CLI Tool!");
            println!("This tool helps you organize and manage your weekly meal plans.");
            println!("Use --help to see available commands.");
        }
    }

    println!("Default storage path: {:?}", config.meal_plan_storage_path);
}

fn remove_meal(meal_plan: &mut MealPlan, meal_type_str: String, day_str: String) -> Result<(), String> {
    // Validate meal type
    let meal_type = match meal_type_str.to_lowercase().as_str() {
        "breakfast" => MealType::Breakfast,
        "lunch" => MealType::Lunch,
        "dinner" => MealType::Dinner,
        "snack" => MealType::Snack,
        _ => return Err("Invalid meal type. Must be breakfast, lunch, dinner, or snack.".to_string()),
    };

    // Validate day
    let day = parse_day(&day_str)?;

    // Check if the meal exists
    if meal_plan.find_meal(&meal_type, &day).is_none() {
        return Err(format!("No {} meal found for {}.", meal_type, day));
    }

    // Check if this is the last meal in the plan
    if meal_plan.meals.len() == 1 {
        println!("This is the last meal in your plan. Are you sure you want to remove it? (y/n)");
        if !confirm() {
            return Err("Meal removal cancelled by user.".to_string());
        }
    }

    // Remove the meal
    meal_plan.remove_meal(&meal_type, &day);
    Ok(())
}

fn edit_meal(meal_plan: &mut MealPlan, meal_type_str: String, day_str: String, new_cook: Option<String>, new_description: Option<String>) -> Result<(), String> {
    // Validate meal type
    let meal_type = match meal_type_str.to_lowercase().as_str() {
        "breakfast" => MealType::Breakfast,
        "lunch" => MealType::Lunch,
        "dinner" => MealType::Dinner,
        "snack" => MealType::Snack,
        _ => return Err("Invalid meal type. Must be breakfast, lunch, dinner, or snack.".to_string()),
    };

    // Validate day
    let day = parse_day(&day_str)?;

    // Find the meal to edit
    let meal = meal_plan.find_meal(&meal_type, &day)
        .ok_or_else(|| format!("No {} meal found for {}.", meal_type, day))?;

    // Display current meal details
    println!("Current meal details:");
    println!("  Type: {}", meal.meal_type);
    println!("  Day: {}", meal.day);
    println!("  Cook: {}", meal.cook);
    println!("  Description: {}", meal.description);
    println!();

    // Get updated values from user
    let new_cook = if let Some(cook) = new_cook {
        cook
    } else {
        println!("Enter new cook (leave empty to keep current value):");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).map_err(|e| e.to_string())?;
        let input = input.trim();
        if input.is_empty() {
            meal.cook.clone()
        } else {
            input.to_string()
        }
    };

    let new_description = if let Some(desc) = new_description {
        desc
    } else {
        println!("Enter new description (leave empty to keep current value):");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).map_err(|e| e.to_string())?;
        let input = input.trim();
        if input.is_empty() {
            meal.description.clone()
        } else {
            input.to_string()
        }
    };

    // Remove the old meal and add the updated one
    meal_plan.remove_meal(&meal_type, &day);
    let updated_meal = Meal::new(meal_type, day, new_cook, new_description);
    meal_plan.add_meal(updated_meal);

    Ok(())
}

fn add_meal(meal_plan: &mut MealPlan, meal_type: String, day: String, cook: String, description: String) -> Result<(), String> {
    // Validate meal type
    let meal_type = match meal_type.to_lowercase().as_str() {
        "breakfast" => MealType::Breakfast,
        "lunch" => MealType::Lunch,
        "dinner" => MealType::Dinner,
        "snack" => MealType::Snack,
        _ => return Err("Invalid meal type. Must be breakfast, lunch, dinner, or snack.".to_string()),
    };

    // Validate day
    let day = parse_day(&day)?;

    // Check for duplicate meals
    if meal_plan.find_meal(&meal_type, &day).is_some() {
        println!("A meal of this type already exists for this day. Do you want to replace it? (y/n)");
        if !confirm() {
            return Err("Meal not added due to user cancellation.".to_string());
        }
        meal_plan.remove_meal(&meal_type, &day);
    }

    // Add the new meal
    let new_meal = Meal::new(meal_type, day, cook, description);
    meal_plan.add_meal(new_meal);

    Ok(())
}

fn parse_day(day_str: &str) -> Result<Day, String> {
    // Try parsing as a date first
    if let Ok(date) = NaiveDate::parse_from_str(day_str, "%Y-%m-%d") {
        return Ok(Day::Date(date));
    }

    // If not a date, try parsing as a weekday
    match day_str.to_lowercase().as_str() {
        "monday" => Ok(Day::Weekday(Weekday::Mon)),
        "tuesday" => Ok(Day::Weekday(Weekday::Tue)),
        "wednesday" => Ok(Day::Weekday(Weekday::Wed)),
        "thursday" => Ok(Day::Weekday(Weekday::Thu)),
        "friday" => Ok(Day::Weekday(Weekday::Fri)),
        "saturday" => Ok(Day::Weekday(Weekday::Sat)),
        "sunday" => Ok(Day::Weekday(Weekday::Sun)),
        _ => Err("Invalid day format. Use YYYY-MM-DD or day name.".to_string()),
    }
}

fn export_ical(meal_plan: &MealPlan, output_path: &PathBuf) -> Result<(), String> {
    use icalendar::{Calendar, Component, Event, Property};
    use chrono::{Duration, TimeZone, Utc};
    
    // Create a new calendar
    let mut calendar = Calendar::new();
    
    // Add calendar properties
    calendar.push(Property::new("PRODID", "-//Meal Plan CLI//EN"));
    calendar.push(Property::new("VERSION", "2.0"));
    calendar.push(Property::new("CALSCALE", "GREGORIAN"));
    
    // Add events for each meal
    for meal in &meal_plan.meals {
        // Create a new event
        let mut event = Event::new();
        
        // Set event properties
        let summary = format!("{}: {}", meal.meal_type, meal.description);
        event.push(Property::new("SUMMARY", summary));
        
        let description = format!("Cook: {}", meal.cook);
        event.push(Property::new("DESCRIPTION", description));
        
        // Set date/time
        let date = match &meal.day {
            Day::Weekday(weekday) => {
                // Find the next occurrence of this weekday from the week start date
                let days_to_add = (*weekday as i64 - meal_plan.week_start_date.weekday().num_days_from_monday() as i64)
                    .rem_euclid(7);
                meal_plan.week_start_date + Duration::days(days_to_add)
            },
            Day::Date(date) => *date,
        };
        
        // Set meal time based on meal type (approximate times)
        let (hour, minute) = match meal.meal_type {
            MealType::Breakfast => (8, 0),
            MealType::Lunch => (12, 0),
            MealType::Dinner => (18, 0),
            MealType::Snack => (15, 0),
        };
        
        // Create start and end times (1 hour duration)
        let start_time = Utc.with_ymd_and_hms(
            date.year(), date.month(), date.day(), 
            hour, minute, 0
        ).unwrap();
        
        let end_time = start_time + Duration::hours(1);
        
        event.push(Property::new("DTSTART", start_time.format("%Y%m%dT%H%M%SZ").to_string()));
        event.push(Property::new("DTEND", end_time.format("%Y%m%dT%H%M%SZ").to_string()));
        
        // Add a unique identifier
        let uid = format!("meal-{}-{}-{:?}@mealplan", 
            meal.meal_type.to_string().to_lowercase(),
            date.format("%Y%m%d"),
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()
        );
        event.push(Property::new("UID", uid));
        
        // Add the event to the calendar
        calendar.push(event);
    }
    
    // Write the calendar to file
    let ical_string = calendar.to_string();
    std::fs::write(output_path, ical_string)
        .map_err(|e| format!("Failed to write iCal file: {}", e))?;
    
    Ok(())
}

fn export_json(meal_plan: &MealPlan, output_path: &PathBuf) -> Result<(), String> {
    // Simply use the existing save_to_json method
    meal_plan.save_to_json(output_path)
        .map_err(|e| format!("Failed to export meal plan to JSON: {}", e))
}

fn confirm() -> bool {
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_lowercase() == "y"
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;
    use std::io::Write;

    #[test]
    fn verify_cli() {
        Args::command().debug_assert()
    }

    #[test]
    fn test_add_command() {
        let args = Args::parse_from(&[
            "mealplan",
            "add",
            "Spaghetti Bolognese",
            "--meal-type", "Dinner",
            "--day", "Monday",
            "--cook", "John",
        ]);
        match args.command {
            Some(Commands::Add { description, meal_type, day, cook }) => {
                assert_eq!(description, "Spaghetti Bolognese");
                assert_eq!(meal_type, "Dinner");
                assert_eq!(day, "Monday");
                assert_eq!(cook, "John");
            }
            _ => panic!("Expected Add command"),
        }
    }

    #[test]
    fn test_edit_command() {
        let args = Args::parse_from(&[
            "mealplan",
            "edit",
            "Updated meal description",
            "--meal-type", "Lunch",
            "--day", "Tuesday",
        ]);
        match args.command {
            Some(Commands::Edit { description, meal_type, day, cook }) => {
                assert_eq!(description, Some("Updated meal description".to_string()));
                assert_eq!(meal_type, "Lunch");
                assert_eq!(day, "Tuesday");
                assert_eq!(cook, None);
            }
            _ => panic!("Expected Edit command"),
        }
    }

    #[test]
    fn test_remove_command() {
        let args = Args::parse_from(&[
            "mealplan",
            "remove",
            "--meal-type", "Breakfast",
            "--day", "Wednesday"
        ]);
        match args.command {
            Some(Commands::Remove { meal_type, day }) => {
                assert_eq!(meal_type, "Breakfast");
                assert_eq!(day, "Wednesday");
            }
            _ => panic!("Expected Remove command"),
        }
    }

    #[test]
    fn test_export_ical_command() {
        let args = Args::parse_from(&[
            "mealplan",
            "export-ical",
            "--output", "/tmp/mealplan.ics"
        ]);
        match args.command {
            Some(Commands::ExportIcal { output }) => {
                assert_eq!(output, PathBuf::from("/tmp/mealplan.ics"));
            }
            _ => panic!("Expected ExportIcal command"),
        }
    }

    #[test]
    fn test_config_init_command() {
        let args = Args::parse_from(&[
            "mealplan",
            "config",
            "init"
        ]);
        match args.command {
            Some(Commands::Config { action: ConfigAction::Init }) => {},
            _ => panic!("Expected Config Init command"),
        }
    }

    #[test]
    fn test_add_meal() {
        let mut meal_plan = MealPlan::new(Local::now().date_naive());
        
        // Test adding a valid meal
        assert!(add_meal(&mut meal_plan, "Dinner".to_string(), "Monday".to_string(), "John".to_string(), "Pasta".to_string()).is_ok());
        
        // Test adding an invalid meal type
        assert!(add_meal(&mut meal_plan, "Brunch".to_string(), "Tuesday".to_string(), "Alice".to_string(), "Eggs".to_string()).is_err());
        
        // Test adding a meal with an invalid day
        assert!(add_meal(&mut meal_plan, "Lunch".to_string(), "Someday".to_string(), "Bob".to_string(), "Sandwich".to_string()).is_err());
        
        // Test adding a duplicate meal (this would normally prompt the user, but in tests it will just fail)
        assert!(add_meal(&mut meal_plan, "Dinner".to_string(), "Monday".to_string(), "Jane".to_string(), "Pizza".to_string()).is_err());
    }

    #[test]
    fn test_edit_meal() {
        let mut meal_plan = MealPlan::new(Local::now().date_naive());
        
        // Add a meal first
        add_meal(&mut meal_plan, "Dinner".to_string(), "Monday".to_string(), "John".to_string(), "Pasta".to_string()).unwrap();
        
        // Test editing a non-existent meal
        assert!(edit_meal(&mut meal_plan, "Breakfast".to_string(), "Monday".to_string(), Some("Alice".to_string()), None).is_err());
        
        // Test editing with invalid meal type
        assert!(edit_meal(&mut meal_plan, "Brunch".to_string(), "Monday".to_string(), Some("Alice".to_string()), None).is_err());
        
        // Test editing with invalid day
        assert!(edit_meal(&mut meal_plan, "Dinner".to_string(), "Someday".to_string(), Some("Alice".to_string()), None).is_err());
        
        // Test successful edit with provided values (no interactive prompts)
        assert!(edit_meal(&mut meal_plan, "Dinner".to_string(), "Monday".to_string(), 
                         Some("Alice".to_string()), Some("Updated pasta dish".to_string())).is_ok());
        
        // Verify the meal was updated
        let updated_meal = meal_plan.find_meal(&MealType::Dinner, &Day::Weekday(Weekday::Mon)).unwrap();
        assert_eq!(updated_meal.cook, "Alice");
        assert_eq!(updated_meal.description, "Updated pasta dish");
    }

    #[test]
    fn test_remove_meal() {
        let mut meal_plan = MealPlan::new(Local::now().date_naive());
        
        // Test removing a non-existent meal
        assert!(remove_meal(&mut meal_plan, "Breakfast".to_string(), "Monday".to_string()).is_err());
        
        // Test removing with invalid meal type
        assert!(remove_meal(&mut meal_plan, "Brunch".to_string(), "Monday".to_string()).is_err());
        
        // Test removing with invalid day
        assert!(remove_meal(&mut meal_plan, "Dinner".to_string(), "Someday".to_string()).is_err());
        
        // Add a meal first
        add_meal(&mut meal_plan, "Dinner".to_string(), "Monday".to_string(), "John".to_string(), "Pasta".to_string()).unwrap();
        
        // Test successful removal
        assert!(remove_meal(&mut meal_plan, "Dinner".to_string(), "Monday".to_string()).is_ok());
        
        // Verify the meal was removed
        assert!(meal_plan.find_meal(&MealType::Dinner, &Day::Weekday(Weekday::Mon)).is_none());
        
        // Add multiple meals to test the last meal confirmation
        add_meal(&mut meal_plan, "Breakfast".to_string(), "Monday".to_string(), "Alice".to_string(), "Cereal".to_string()).unwrap();
        add_meal(&mut meal_plan, "Lunch".to_string(), "Monday".to_string(), "Bob".to_string(), "Sandwich".to_string()).unwrap();
        
        // Remove one meal, should succeed without confirmation (not the last meal)
        assert!(remove_meal(&mut meal_plan, "Breakfast".to_string(), "Monday".to_string()).is_ok());
        
        // Verify only one meal remains
        assert_eq!(meal_plan.meals.len(), 1);
        
        // The last meal removal would normally prompt for confirmation
        // In tests, we can't easily simulate user input, so we'll skip testing that specific case
    }

    #[test]
    fn test_parse_day() {
        assert!(matches!(parse_day("2023-05-01"), Ok(Day::Date(_))));
        assert!(matches!(parse_day("Monday"), Ok(Day::Weekday(Weekday::Mon))));
        assert!(parse_day("Invalid").is_err());
    }
    
    #[test]
    fn test_export_json() {
        let mut meal_plan = MealPlan::new(Local::now().date_naive());
        
        // Add a meal
        add_meal(&mut meal_plan, "Dinner".to_string(), "Monday".to_string(), "John".to_string(), "Pasta".to_string()).unwrap();
        
        // Create a temporary file for testing
        let temp_dir = tempfile::tempdir().unwrap();
        let output_path = temp_dir.path().join("test_export.json");
        
        // Export to JSON
        assert!(export_json(&meal_plan, &output_path).is_ok());
        
        // Verify the file exists
        assert!(output_path.exists());
        
        // Load the exported file and verify contents
        let loaded_plan = MealPlan::load_from_json(&output_path).unwrap();
        assert_eq!(loaded_plan.meals.len(), 1);
        assert_eq!(loaded_plan.meals[0].description, "Pasta");
    }
    
    #[test]
    fn test_export_ical() {
        let mut meal_plan = MealPlan::new(Local::now().date_naive());
        
        // Add a meal
        add_meal(&mut meal_plan, "Dinner".to_string(), "Monday".to_string(), "John".to_string(), "Pasta".to_string()).unwrap();
        
        // Create a temporary file for testing
        let temp_dir = tempfile::tempdir().unwrap();
        let output_path = temp_dir.path().join("test_export.ics");
        
        // Export to iCal
        assert!(export_ical(&meal_plan, &output_path).is_ok());
        
        // Verify the file exists
        assert!(output_path.exists());
        
        // Read the file and check for expected iCal format elements
        let content = std::fs::read_to_string(&output_path).unwrap();
        assert!(content.contains("BEGIN:VCALENDAR"));
        assert!(content.contains("VERSION:2.0"));
        assert!(content.contains("BEGIN:VEVENT"));
        assert!(content.contains("SUMMARY:Dinner: Pasta"));
        assert!(content.contains("DESCRIPTION:Cook: John"));
        assert!(content.contains("END:VEVENT"));
        assert!(content.contains("END:VCALENDAR"));
    }
}
