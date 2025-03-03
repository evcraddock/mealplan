mod models;

use clap::{Parser, Subcommand};
use models::Config;
use std::path::PathBuf;

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
        #[arg(short = 't', long)]
        meal_type: String,
        #[arg(short, long)]
        day: String,
        #[arg(short, long)]
        cook: String,
    },
    /// Edit an existing meal in the plan
    Edit {
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

    match args.command {
        Some(Commands::Add { meal_type, day, cook }) => {
            println!("Adding meal: {} on {} cooked by {}", meal_type, day, cook);
            // TODO: Implement add_meal function
        }
        Some(Commands::Edit { meal_type, day, cook }) => {
            println!("Editing meal: {} on {}", meal_type, day);
            if let Some(c) = cook {
                println!("New cook: {}", c);
            }
            // TODO: Implement edit_meal function
        }
        Some(Commands::Remove { meal_type, day }) => {
            println!("Removing meal: {} on {}", meal_type, day);
            // TODO: Implement remove_meal function
        }
        Some(Commands::ExportIcal { output }) => {
            println!("Exporting meal plan to iCal: {:?}", output);
            // TODO: Implement export_ical function
        }
        Some(Commands::ExportJson { output }) => {
            println!("Exporting meal plan to JSON: {:?}", output);
            // TODO: Implement export_json function
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

    // Initialize default configuration
    let config = Config::new();
    println!("Default storage path: {:?}", config.meal_plan_storage_path);
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        Args::command().debug_assert()
    }

    #[test]
    fn test_add_command() {
        let args = Args::parse_from(&[
            "mealplan",
            "add",
            "--meal-type", "Dinner",
            "--day", "Monday",
            "--cook", "John",
        ]);
        match args.command {
            Some(Commands::Add { meal_type, day, cook }) => {
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
            "--meal-type", "Lunch",
            "--day", "Tuesday",
        ]);
        match args.command {
            Some(Commands::Edit { meal_type, day, cook }) => {
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

    // Existing model tests...
    use crate::models::{Day, Meal, MealType};
    use chrono::{NaiveDate, Weekday};
    
    #[test]
    fn test_project_compiles() {
        // This test will pass if the project compiles successfully
        assert!(true);
    }
    
    #[test]
    fn test_models_integration() {
        // Create a meal plan
        let week_start = NaiveDate::from_ymd_opt(2023, 1, 2).unwrap();
        let mut plan = MealPlan::new(week_start);
        
        // Add a meal
        let meal = Meal::new(
            MealType::Dinner,
            Day::Weekday(Weekday::Fri),
            "Test Cook".to_string(),
        );
        plan.add_meal(meal);
        
        // Verify meal was added
        assert_eq!(plan.meals.len(), 1);
        assert_eq!(plan.meals[0].cook, "Test Cook");
    }

    #[test]
    fn test_meal_plan_creation() {
        let start_date = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        let meal_plan = MealPlan::new(start_date);
        assert_eq!(meal_plan.week_start_date, start_date);
        assert!(meal_plan.meals.is_empty());
    }
}
