# **Meal Plan CLI Tool Specification**

## **Overview**

The Meal Plan CLI Tool allows users to create, edit, and manage weekly meal plans. It operates via command-line commands and supports meal plans stored in markdown files and JSON format. The tool generates iCalendar files for scheduling meals and enables meal plan synchronization between markdown and JSON files.

## **Functional Requirements**

### **1. Command Structure**

- **add**: Adds a meal to the meal plan for a specific day and meal type. Example:

  ```sh

  mealplan add "breakfast" "Monday" "Erik" "Bacon and Eggs"
  ```

  - Days can be specified by name (e.g., “Monday”) or date in a format supported by Rust.
  - The tool assumes the upcoming Monday if a day name is provided.
  - The cook must always be specified.
  - The tool will validate meal types (breakfast, lunch, dinner, snack) and prevent duplicates within the same day.

- **edit**: Edits a meal. Displays current details and prompts for changes.

  ```sh
  mealplan edit "breakfast" "Monday"
  ```

- **remove**: Removes a meal by specifying the meal type and day.

  ```sh
  mealplan remove "breakfast" "Monday"
  ```

  - The tool will prompt for confirmation before removing the last meal for a week.

- **export-ical**: Generates an iCalendar file for the week’s meal plan.

  ```sh
  mealplan export-ical
  ```

- **export-json**: Exports the meal plan to a JSON file.

  ```sh
  mealplan export-json
  ```

- **sync**: Synchronizes the JSON and markdown files, regenerating the markdown file from the most recent data source.

  ```sh
  mealplan sync
  ```

- **config init**: Initializes the config file with default settings.

  ```sh
  mealplan config init
  ```

- **--path**: Allows overriding the meal plan storage location for testing.

  ```sh
  mealplan add --path /custom/location "breakfast" "Monday" "Erik" "Bacon and Eggs"
  ```

- **--version**: Displays the version number of the tool.

  ```sh
  mealplan --version
  ```

- **--help**: Displays usage instructions for commands.

### **2. Data Handling**

- **File Storage**:
  - The meal plan will be stored in a structured folder system under `~/.config/todufit/` using a format like `2025/01-January/01-01-2025/mealplan.md`.
  - Each week’s meal plan will have a separate markdown file (`mealplan.md`).
  - The iCalendar file will be stored as `mealplan.ics` in the root folder defined in the config.
  - The JSON export will be stored as `mealplan.json` in the same folder as the markdown file.

- **Meal Types**: The following meal types are supported:
  - **Breakfast**
  - **Lunch**
  - **Dinner**
  - **Snack**

- **Date Handling**:
  - Meal plans are organized by week, starting from **Sunday** (system date).
  - The user may specify dates or use day names (e.g., “Monday”) for meal entries.
  - Only future meals can be added; the tool prevents adding meals to past dates.

- **Meal Plan File Sync**:
  - The tool will sync the JSON and markdown files by checking the most recently updated file. The other file is regenerated accordingly.
  - The tool assumes the JSON file is correct but does not validate it on startup.

### **3. Error Handling**

- **Meal Duplication**: The tool will prompt users to confirm before adding duplicate meals for the same day.
- **Meal Removal**: If deleting the last meal of the week, the tool will display a warning before proceeding.
- **Invalid Date/Meal Type**: The tool will validate the meal type and prevent entry of invalid types. It will also reject past dates and invalid formats.
- **File Overwriting**: If a file exists in the specified location (e.g., `mealplan.md`), the tool will warn the user before overwriting.

### **4. Testing Plan**

- **Unit Tests**:
  - Test individual commands (`add`, `edit`, `remove`) for handling valid inputs and edge cases (e.g., duplicates, past dates).
  - Test JSON and markdown synchronization, ensuring the files are correctly regenerated.
  - Test iCalendar and JSON exports for correct formatting.
  - Test error handling for invalid meal types, duplicate entries, and out-of-bound dates.

- **Integration Tests**:
  - Test end-to-end scenarios, such as adding a meal, exporting iCalendar, editing a meal, and syncing the meal plan.

- **Manual Testing**:
  - Test the `mealplan sync` command after editing the JSON file manually.
  - Test with edge cases (e.g., adding multiple meals for the same day, using different date formats).

### **5. Architecture Choices**

- **Language**: Rust will be used to develop the tool, utilizing its strong handling of dates, file I/O, and performance benefits for CLI tools.
- **Data Format**: Markdown will be used for meal plan storage, with JSON as the primary data format for internal processing and synchronization.
- **Error Messages**: Errors will be displayed on-screen with clear, concise messages. No error codes will be returned, as the tool uses simple confirmation prompts.

### **6. Configuration**

- The configuration file (`~/.config/todufit/config.toml`) will store the following settings:
  - `meal_plan_storage_path`: Default path where meal plans are stored (can be overridden via the `--path` flag).
  - `current_week_start_date`: Start date for the current week’s meal plan (calculated automatically based on the system date).
- The config file will be manually edited if changes are required.

### **7. Additional Features (Future Enhancements)**

- Meal suggestions or templates could be added later.
- Support for meal plan importing and exporting from other apps may be considered.
- The ability to specify recurring meal types or recurring cooks could be implemented in future versions.
