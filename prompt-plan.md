# Prompt 1: Project Setup

Task: Create a new Rust project for the Meal Plan CLI Tool.
Steps:

- Add the following dependencies in `Cargo.toml`:
  - `clap` for command-line argument parsing.
  - `serde` and `serde_json` for JSON handling.
  - `chrono` for date handling.
  - An appropriate crate for iCalendar file generation (e.g., `icalendar`).
- Implement a basic `main` function that prints a welcome message.
- Write a simple unit test to verify that the project compiles.
End with wiring everything together in `main.rs`.

## Prompt 2: Data Models and Serialization

Task: Define the core data models for the Meal Plan CLI Tool.
Steps:

- Create a `Meal` struct with fields for:
  - `meal_type` (as an enum: breakfast, lunch, dinner, snack)
  - `day` (which can be a day name or a date)
  - `cook` (the person responsible)
  - `description` (the meal details)
- Create a `MealPlan` struct to represent a week’s meal plan (e.g., a collection of `Meal` entries).
- Create a `Config` struct to store configuration settings such as `meal_plan_storage_path` and `current_week_start_date`.
- Derive necessary traits (e.g., `Serialize`, `Deserialize`, `Debug`) for these structs.
- Implement functions for serializing/deserializing the meal plan to/from JSON and for reading/writing markdown.
- Write unit tests for serialization and file I/O operations.
End with integrating these models into a dedicated module.

## Prompt 3: Command-Line Interface (CLI) Parsing

Task: Implement the CLI command parsing using Clap.
Steps:

- Define the CLI commands and options as per the project spec:
  - Subcommands: `add`, `edit`, `remove`, `export-ical`, `export-json`, `sync`, `config init`
  - Global options: `--path`, `--version`, and `--help`
- Create a function that parses the command-line arguments and maps them to a command enum or structure.
- Validate required arguments and provide helpful error messages for missing or invalid inputs.
- Write unit tests to verify that the CLI parsing works as expected for different commands.
End with wiring the parsed commands to their corresponding handler functions.

## Prompt 4: Implement "add" Command

Task: Implement the "add" command to add a meal to the meal plan.
Steps:

- Create a function to handle the `add` command.
- Validate input arguments:
  - Check that the meal type is one of the allowed values (breakfast, lunch, dinner, snack).
  - Validate the day (either as a day name or a valid date).
  - Ensure a cook and a description are provided.
- Implement logic to check for duplicate meals on the same day and prompt the user for confirmation if a duplicate exists.
- Integrate file I/O to update the meal plan storage (updating both markdown and JSON as needed).
- Write unit tests covering:
  - Successful meal addition.
  - Handling of duplicates.
  - Edge cases (e.g., invalid meal types or past dates).
End with wiring the "add" command handler into the overall CLI command structure.

## Prompt 5: Implement "edit" Command

Task: Implement the "edit" command to modify an existing meal.
Steps:

- Create a function to handle the `edit` command.
- Retrieve the current meal details based on meal type and day.
- Display the current details and prompt the user for updated values.
- Validate the new input and update the meal entry accordingly.
- Ensure the updated meal is persisted to storage (both markdown and JSON).
- Write unit tests to cover:
  - Editing an existing meal.
  - Handling cases where the meal does not exist.
  - Validating new input values.
End with wiring the "edit" command handler into the CLI parsing.

## Prompt 6: Implement "remove" Command

Task: Implement the "remove" command for deleting a meal.
Steps:

- Create a function to handle the `remove` command.
- Retrieve the meal to be removed using the meal type and day.
- If the meal is the last one in the week, display a warning and ask for confirmation.
- Remove the meal from the meal plan and update the storage files.
- Write unit tests covering:
  - Successful removal.
  - Removal confirmation when deleting the last meal of the week.
  - Handling of non-existent meals.
End with wiring the "remove" command handler into the CLI.

## Prompt 7: Implement Export Commands

Task: Implement export functionalities for iCalendar and JSON.
Steps:

- For `export-ical`:
  - Create a function that converts the current week’s meal plan into an iCalendar (.ics) file using an appropriate crate.
  - Validate that the generated file follows the iCalendar format.
- For `export-json`:
  - Create a function that serializes the meal plan into JSON format and writes it to the storage location.
- Write unit tests to ensure:
  - The iCalendar file is generated with correct event details.
  - The JSON file correctly reflects the meal plan data.
End with wiring the export command handlers into the CLI command structure.

## Prompt 8: Implement "sync" Command

Task: Implement the "sync" command to synchronize the JSON and markdown meal plan files.
Steps:

- Create a function to handle the `sync` command.
- Implement logic to determine which file (JSON or markdown) is the most recently updated.
- Regenerate the out-of-date file from the most recent data source.
- Validate that the synchronization maintains data consistency between the files.
- Write unit tests for various sync scenarios:
  - JSON is more recent than markdown.
  - Markdown is more recent than JSON.
  - Handling of errors during file read/write.
End with wiring the "sync" command handler into the overall CLI.

## Prompt 9: Implement "config init" Command

Task: Implement the configuration initialization command.
Steps:

- Create a function to handle the `config init` command.
- Generate a default configuration file (`config.toml`) with:
  - Default `meal_plan_storage_path`
  - Calculated `current_week_start_date`
- Ensure that the configuration file is created in `~/.config/todufit/`. If it already exists, prompt the user for confirmation before overwriting.
- Write unit tests to validate:
  - Correct creation of the config file.
  - Proper handling when the file already exists.
End with wiring the configuration command handler into the CLI.

## Prompt 10: Integration and End-to-End Testing

Task: Integrate all components and ensure robust error handling and thorough testing.
Steps:

- Combine CLI parsing, command handlers (add, edit, remove, export-ical, export-json, sync, config init), and data models in the `main` function.
- Implement global error handling to catch and display errors clearly.
- Develop integration tests to simulate complete end-to-end scenarios, such as:
  - Adding a meal, editing it, exporting, and then syncing.
  - Testing various user flows and edge cases.
- Document manual testing instructions for:
  - Verifying CLI usage.
  - Checking file outputs (markdown, JSON, iCalendar).
End with wiring everything together in `main.rs`, ensuring that all features are integrated seamlessly and that no orphaned code remains.
