mod models;

use clap::Parser;
use models::{Config, MealPlan};

/// Meal Plan CLI Tool
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Command line arguments will be added here later
}

fn main() {
    let _args = Args::parse();
    println!("Welcome to the Meal Plan CLI Tool!");
    println!("This tool helps you organize and manage your weekly meal plans.");
    
    // Initialize default configuration
    let config = Config::new();
    println!("Default storage path: {:?}", config.meal_plan_storage_path);
}

#[cfg(test)]
mod tests {
    use super::*;
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
            "Test Meal".to_string(),
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
