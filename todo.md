# Meal Plan CLI Tool Checklist

## Project Setup

- [x] **Initialize Cargo Project**
  - [x] Run `cargo new mealplan` to create a new project.
  - [x] Set up the basic project structure.
- [x] **Configure Dependencies in Cargo.toml**
  - [x] Add `clap` for CLI parsing.
  - [x] Add `serde` and `serde_json` for JSON serialization.
  - [x] Add `chrono` for date handling.
  - [x] Add an iCalendar crate (e.g., `icalendar`).
- [x] **Basic Main Function**
  - [x] Implement a simple `main.rs` that prints a welcome message.
  - [x] Write a basic unit test to ensure the project compiles.

## Data Models and Serialization

- [x] **Define Data Structures**
  - [x] Create a `Meal` struct with fields:
    - [x] `meal_type` (enum: breakfast, lunch, dinner, snack)
    - [x] `day` (string/date handling)
    - [x] `cook` (string)
    - [x] `description` (string)
  - [x] Create a `MealPlan` struct to represent a week's meals.
  - [x] Create a `Config` struct for settings (e.g., storage path, week start date).
- [x] **Implement Traits**
  - [x] Derive `Serialize`, `Deserialize`, and `Debug` for each struct.
- [x] **File I/O and Serialization Functions**
  - [x] Implement functions to serialize and deserialize meal plans to/from JSON.
  - [x] Implement functions to read and write meal plans in Markdown.
- [x] **Unit Testing for Data Models**
  - [x] Write tests for serialization/deserialization.
  - [x] Write tests for file I/O operations.

## Command-Line Interface (CLI) Parsing

- [x] **Define CLI Commands and Options**
  - [x] Set up subcommands: `add`, `edit`, `remove`, `export-ical`, `export-json`, `sync`, `config init`.
  - [x] Set up global options: `--path`, `--version`, `--help`.
- [x] **Implement CLI Parsing with Clap**
  - [x] Map CLI arguments to command enums or structures.
  - [x] Validate required arguments and provide error messages.
- [x] **Unit Tests for CLI Parsing**
  - [x] Create tests for each command and option.
  - [x] Verify error handling for missing/invalid inputs.
- [x] **Wire CLI Parsing to Command Handlers**

## Implementing Commands

### Add Command

- [x] **Implement `add` Command Handler**
  - [x] Validate meal type (breakfast, lunch, dinner, snack).
  - [x] Validate day input (day name or date).
  - [x] Ensure cook and description are provided.
  - [x] Check for duplicate meals on the same day.
  - [x] Prompt for confirmation on duplicates.
  - [x] Update meal plan storage (JSON).
- [x] **Unit Tests for `add` Command**
  - [x] Test successful meal addition.
  - [x] Test duplicate detection and confirmation.
  - [x] Test edge cases (invalid types, invalid days).
- [x] **Wire `add` Handler to CLI**

Note: Future date restriction not implemented yet.

### Edit Command

- [x] **Implement `edit` Command Handler**
  - [x] Retrieve existing meal based on meal type and day.
  - [x] Display current meal details.
  - [x] Prompt for updated values.
  - [x] Validate new input and update the meal entry.
  - [x] Persist changes to Markdown & JSON.
- [x] **Unit Tests for `edit` Command**
  - [x] Test editing an existing meal.
  - [x] Test handling non-existent meals.
  - [x] Validate updated input values.
- [x] **Wire `edit` Handler to CLI**

### Remove Command

- [x] **Implement `remove` Command Handler**
  - [x] Retrieve meal to be removed based on meal type and day.
  - [x] Check if the meal is the last of the week; if so, prompt for confirmation.
  - [x] Remove the meal and update storage.
- [x] **Unit Tests for `remove` Command**
  - [x] Test successful removal.
  - [x] Test confirmation prompt for last meal removal.
  - [x] Test removal of non-existent meals.
- [x] **Wire `remove` Handler to CLI**

### Export Commands

- [x] **Implement `export-ical` Command Handler**
  - [x] Convert current week's meal plan to an iCalendar (.ics) file.
  - [x] Validate the generated iCalendar format.
- [x] **Implement `export-json` Command Handler**
  - [x] Serialize the meal plan to JSON.
  - [x] Write JSON file to the designated storage location.
- [x] **Unit Tests for Export Commands**
  - [x] Test iCalendar generation for correct event details.
  - [x] Test JSON export matches meal plan data.
- [x] **Wire Export Handlers to CLI**

### Sync Command

- [x] **Implement `sync` Command Handler**
  - [x] Compare timestamps of JSON and Markdown files.
  - [x] Regenerate the older file from the most recent data source.
  - [x] Ensure data consistency between files.
- [x] **Unit Tests for `sync` Command**
  - [x] Test scenario where JSON is more recent.
  - [x] Test scenario where Markdown is more recent.
  - [x] Test error handling during file read/write.
- [x] **Wire `sync` Handler to CLI**

### Config Init Command

- [x] **Implement `config init` Command Handler**
  - [x] Generate a default `config.json` in `~/.config/mealplan/`.
  - [x] Set default values (e.g., `meal_plan_storage_path`, `current_week_start_date`).
  - [x] If config file exists, prompt user for confirmation before overwriting.
- [x] **Unit Tests for `config init` Command**
  - [x] Test config file creation.
  - [x] Test handling when the config file already exists.
- [x] **Wire `config init` Handler to CLI**

## Integration and Testing

- [x] **Integrate All Components**
  - [x] Combine CLI parsing, command handlers, and data models in `main.rs`.
  - [x] Implement global error handling to display errors clearly.
- [x] **Develop Integration Tests**
  - [x] End-to-end scenario: add, edit, export, and sync a meal.
  - [x] Simulate complete user flows with edge cases.
- [x] **Document Manual Testing Procedures**
  - [x] Instructions for verifying CLI usage.
  - [x] Steps to check file outputs (Markdown, JSON, iCalendar).

## Final Tasks

- [x] **Code Review and Refactoring**
  - [x] Ensure no orphaned code remains.
  - [x] Confirm that each feature builds on the previous steps.
- [x] **Documentation**
  - [x] Write README.md with project description and usage instructions.
  - [x] Update inline code comments and docs for clarity.
- [x] **Final Testing**
  - [x] Run all unit and integration tests.
  - [x] Verify manual testing procedures.
