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

- [ ] **Implement `remove` Command Handler**
  - [ ] Retrieve meal to be removed based on meal type and day.
  - [ ] Check if the meal is the last of the week; if so, prompt for confirmation.
  - [ ] Remove the meal and update storage.
- [ ] **Unit Tests for `remove` Command**
  - [ ] Test successful removal.
  - [ ] Test confirmation prompt for last meal removal.
  - [ ] Test removal of non-existent meals.
- [ ] **Wire `remove` Handler to CLI**

### Export Commands

- [ ] **Implement `export-ical` Command Handler**
  - [ ] Convert current week's meal plan to an iCalendar (.ics) file.
  - [ ] Validate the generated iCalendar format.
- [ ] **Implement `export-json` Command Handler**
  - [ ] Serialize the meal plan to JSON.
  - [ ] Write JSON file to the designated storage location.
- [ ] **Unit Tests for Export Commands**
  - [ ] Test iCalendar generation for correct event details.
  - [ ] Test JSON export matches meal plan data.
- [ ] **Wire Export Handlers to CLI**

### Sync Command

- [ ] **Implement `sync` Command Handler**
  - [ ] Compare timestamps of JSON and Markdown files.
  - [ ] Regenerate the older file from the most recent data source.
  - [ ] Ensure data consistency between files.
- [ ] **Unit Tests for `sync` Command**
  - [ ] Test scenario where JSON is more recent.
  - [ ] Test scenario where Markdown is more recent.
  - [ ] Test error handling during file read/write.
- [ ] **Wire `sync` Handler to CLI**

### Config Init Command

- [ ] **Implement `config init` Command Handler**
  - [ ] Generate a default `config.toml` in `~/.config/todufit/`.
  - [ ] Set default values (e.g., `meal_plan_storage_path`, `current_week_start_date`).
  - [ ] If config file exists, prompt user for confirmation before overwriting.
- [ ] **Unit Tests for `config init` Command**
  - [ ] Test config file creation.
  - [ ] Test handling when the config file already exists.
- [ ] **Wire `config init` Handler to CLI**

## Integration and Testing

- [ ] **Integrate All Components**
  - [ ] Combine CLI parsing, command handlers, and data models in `main.rs`.
  - [ ] Implement global error handling to display errors clearly.
- [ ] **Develop Integration Tests**
  - [ ] End-to-end scenario: add, edit, export, and sync a meal.
  - [ ] Simulate complete user flows with edge cases.
- [ ] **Document Manual Testing Procedures**
  - [ ] Instructions for verifying CLI usage.
  - [ ] Steps to check file outputs (Markdown, JSON, iCalendar).

## Final Tasks

- [ ] **Code Review and Refactoring**
  - [ ] Ensure no orphaned code remains.
  - [ ] Confirm that each feature builds on the previous steps.
- [ ] **Documentation**
  - [ ] Write README.md with project description and usage instructions.
  - [ ] Update inline code comments and docs for clarity.
- [ ] **Final Testing**
  - [ ] Run all unit and integration tests.
  - [ ] Verify manual testing procedures.
