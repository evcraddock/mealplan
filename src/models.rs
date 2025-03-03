use chrono::{DateTime, Utc, NaiveDate, Weekday};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

/// Represents the type of meal
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MealType {
    Breakfast,
    Lunch,
    Dinner,
    Snack,
}

impl std::fmt::Display for MealType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MealType::Breakfast => write!(f, "Breakfast"),
            MealType::Lunch => write!(f, "Lunch"),
            MealType::Dinner => write!(f, "Dinner"),
            MealType::Snack => write!(f, "Snack"),
        }
    }
}

/// Represents a day, which can be a weekday or a specific date
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Day {
    Weekday(Weekday),
    Date(NaiveDate),
}

impl std::fmt::Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Day::Weekday(weekday) => write!(f, "{:?}", weekday),
            Day::Date(date) => write!(f, "{}", date.format("%Y-%m-%d")),
        }
    }
}

/// Represents a single meal entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meal {
    pub meal_type: MealType,
    pub day: Day,
    pub cook: String,
}

impl Meal {
    /// Creates a new meal
    pub fn new(meal_type: MealType, day: Day, cook: String) -> Self {
        Self {
            meal_type,
            day,
            cook,
        }
    }
}

/// Represents a week's meal plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MealPlan {
    pub meals: Vec<Meal>,
    pub week_start_date: NaiveDate,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub last_modified: DateTime<Utc>,
}

impl MealPlan {
    /// Creates a new empty meal plan
    pub fn new(week_start_date: NaiveDate) -> Self {
        Self {
            meals: Vec::new(),
            week_start_date,
            last_modified: Utc::now(),
        }
    }

    /// Adds a meal to the plan
    pub fn add_meal(&mut self, meal: Meal) {
        self.meals.push(meal);
        self.last_modified = Utc::now();
    }

    /// Removes a meal from the plan
    pub fn remove_meal(&mut self, meal_type: &MealType, day: &Day) -> Option<Meal> {
        if let Some(index) = self.meals.iter().position(|m| &m.meal_type == meal_type && &m.day == day) {
            let meal = self.meals.remove(index);
            self.last_modified = Utc::now();
            Some(meal)
        } else {
            None
        }
    }

    /// Finds a meal in the plan
    pub fn find_meal(&self, meal_type: &MealType, day: &Day) -> Option<&Meal> {
        self.meals.iter().find(|m| &m.meal_type == meal_type && &m.day == day)
    }

    /// Saves the meal plan to a JSON file
    pub fn save_to_json<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        let mut file = File::create(path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    /// Loads a meal plan from a JSON file
    pub fn load_from_json<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let meal_plan: MealPlan = serde_json::from_str(&contents)?;
        Ok(meal_plan)
    }

    /// Saves the meal plan to a Markdown file
    pub fn save_to_markdown<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let mut markdown = format!("# Meal Plan for Week of {}\n\n", self.week_start_date.format("%Y-%m-%d"));
        
        // Group meals by day
        let mut meals_by_day: HashMap<&Day, Vec<&Meal>> = HashMap::new();
        for meal in &self.meals {
            meals_by_day.entry(&meal.day).or_default().push(meal);
        }
        
        // Sort days
        let mut days: Vec<&Day> = meals_by_day.keys().cloned().collect();
        days.sort_by_key(|d| match d {
            Day::Weekday(w) => format!("1{:?}", w),
            Day::Date(date) => format!("0{}", date),
        });
        
        for day in days {
            markdown.push_str(&format!("## {}\n\n", day));
            
            if let Some(meals) = meals_by_day.get(day) {
                for meal in meals {
                    markdown.push_str(&format!("### {}\n", meal.meal_type));
                    markdown.push_str(&format!("- Cook: {}\n\n", meal.cook));
                }
            }
        }
        
        markdown.push_str(&format!("\n*Last modified: {}*", self.last_modified.format("%Y-%m-%d %H:%M:%S")));
        
        let mut file = File::create(path)?;
        file.write_all(markdown.as_bytes())?;
        Ok(())
    }

    /// Loads a meal plan from a Markdown file (basic implementation)
    /// Note: This is a simplified implementation and might not handle all edge cases
    pub fn load_from_markdown<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        // For simplicity, we'll just check if the file exists and then suggest using JSON
        // A full implementation would parse the Markdown structure
        if !path.as_ref().exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Markdown file not found",
            ));
        }
        
        // This is a placeholder. In a real implementation, you would parse the Markdown
        // and extract the meal plan data.
        Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "Loading from Markdown is not fully implemented. Please use JSON format.",
        ))
    }
}

/// Configuration settings for the meal plan application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub meal_plan_storage_path: PathBuf,
    pub current_week_start_date: NaiveDate,
}

impl Config {
    /// Creates a new configuration with default values
    pub fn new() -> Self {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let storage_path = home_dir.join(".config").join("mealplan");
        
        // Create the directory if it doesn't exist
        if !storage_path.exists() {
            fs::create_dir_all(&storage_path).unwrap_or_else(|_| {
                eprintln!("Warning: Could not create directory at {:?}", storage_path);
            });
        }
        
        Self {
            meal_plan_storage_path: storage_path,
            current_week_start_date: Utc::now().date_naive(),
        }
    }

    /// Saves the configuration to a JSON file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        let mut file = File::create(path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    /// Loads the configuration from a JSON file
    pub fn load<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: Config = serde_json::from_str(&contents)?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::ErrorKind;
    use tempfile::tempdir;

    #[test]
    fn test_meal_creation() {
        let meal = Meal::new(
            MealType::Dinner,
            Day::Weekday(Weekday::Mon),
            "John".to_string(),
        );
        
        assert_eq!(meal.meal_type, MealType::Dinner);
        assert_eq!(meal.cook, "John");
        
        match meal.day {
            Day::Weekday(day) => assert_eq!(day, Weekday::Mon),
            _ => panic!("Expected Weekday"),
        }
    }

    #[test]
    fn test_meal_plan_operations() {
        let week_start = NaiveDate::from_ymd_opt(2023, 1, 2).unwrap();
        let mut plan = MealPlan::new(week_start);
        
        // Add a meal
        let meal = Meal::new(
            MealType::Lunch,
            Day::Weekday(Weekday::Wed),
            "Alice".to_string(),
        );
        plan.add_meal(meal);
        
        // Find the meal
        let found = plan.find_meal(&MealType::Lunch, &Day::Weekday(Weekday::Wed));
        assert!(found.is_some());
        assert_eq!(found.unwrap().cook, "Alice");
        
        // Remove the meal
        let removed = plan.remove_meal(&MealType::Lunch, &Day::Weekday(Weekday::Wed));
        assert!(removed.is_some());
        
        // Verify it's gone
        let not_found = plan.find_meal(&MealType::Lunch, &Day::Weekday(Weekday::Wed));
        assert!(not_found.is_none());
    }

    #[test]
    fn test_json_serialization() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test_meal_plan.json");
        
        // Create a meal plan
        let week_start = NaiveDate::from_ymd_opt(2023, 1, 2).unwrap();
        let mut plan = MealPlan::new(week_start);
        
        let meal = Meal::new(
            MealType::Breakfast,
            Day::Date(NaiveDate::from_ymd_opt(2023, 1, 3).unwrap()),
            "Bob".to_string(),
        );
        plan.add_meal(meal);
        
        // Save to JSON
        plan.save_to_json(&file_path).unwrap();
        
        // Load from JSON
        let loaded_plan = MealPlan::load_from_json(&file_path).unwrap();
        
        // Verify data
        assert_eq!(loaded_plan.week_start_date, week_start);
        assert_eq!(loaded_plan.meals.len(), 1);
        assert_eq!(loaded_plan.meals[0].cook, "Bob");
    }

    #[test]
    fn test_markdown_export() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test_meal_plan.md");
        
        // Create a meal plan
        let week_start = NaiveDate::from_ymd_opt(2023, 1, 2).unwrap();
        let mut plan = MealPlan::new(week_start);
        
        let meal1 = Meal::new(
            MealType::Breakfast,
            Day::Weekday(Weekday::Mon),
            "Charlie".to_string(),
        );
        plan.add_meal(meal1);
        
        let meal2 = Meal::new(
            MealType::Dinner,
            Day::Weekday(Weekday::Mon),
            "Diana".to_string(),
        );
        plan.add_meal(meal2);
        
        // Save to Markdown
        plan.save_to_markdown(&file_path).unwrap();
        
        // Verify file exists
        assert!(file_path.exists());
        
        // Read the file content to verify it contains expected text
        let content = fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("# Meal Plan for Week of 2023-01-02"));
        assert!(content.contains("## Mon"));
        assert!(content.contains("### Breakfast"));
        assert!(content.contains("- Cook: Charlie"));
        assert!(content.contains("### Dinner"));
        assert!(content.contains("- Cook: Diana"));
    }

    #[test]
    fn test_markdown_import_not_found() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("nonexistent.md");
        
        let result = MealPlan::load_from_markdown(&file_path);
        assert!(result.is_err());
        
        let err = result.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::NotFound);
    }

    #[test]
    fn test_config() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("config.json");
        
        // Create default config
        let config = Config::new();
        
        // Save config
        config.save(&file_path).unwrap();
        
        // Load config
        let loaded_config = Config::load(&file_path).unwrap();
        
        // Verify paths match
        assert_eq!(loaded_config.meal_plan_storage_path, config.meal_plan_storage_path);
    }
}
