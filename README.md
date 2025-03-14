# Meal Plan CLI Tool

A command-line tool for managing weekly meal plans.

## Features

- Add, edit, and remove meals from your weekly plan
- Export meal plans to iCalendar (.ics) format for calendar integration
- Export meal plans to JSON for data portability
- Sync between JSON and Markdown formats
- Configure storage locations and other settings

## Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/mealplan.git
cd mealplan

# Build the project
cargo build --release

# Install the binary (optional)
cargo install --path .
```

## Usage

### Configuration

Initialize the configuration:

```bash
mealplan config init
```

This creates a configuration file at `~/.config/mealplan/config.json`.

### Adding a Meal

```bash
mealplan add "Spaghetti Bolognese" --meal-type dinner --day monday --cook "John Doe"
```

Valid meal types: breakfast, lunch, dinner, snack
Valid days: Monday-Sunday or YYYY-MM-DD format

### Editing a Meal

```bash
mealplan edit "Updated Meal Description" --meal-type dinner --day monday --cook "Jane Doe"
```

### Removing a Meal

```bash
mealplan remove --meal-type dinner --day monday
```

### Exporting to iCalendar

```bash
mealplan export-ical --output meal_plan.ics
```

### Exporting to JSON

```bash
mealplan export-json --output meal_plan.json
```

### Syncing Between Formats

```bash
mealplan sync
```

By default, this uses the most recently modified file as the source. You can specify a source:

```bash
mealplan sync --source json
mealplan sync --source markdown
```

### Using a Custom Storage Path

All commands support a global `--path` option to specify a custom storage location:

```bash
mealplan --path /custom/path add "Meal Description" --meal-type lunch --day tuesday --cook "Chef"
```

## File Locations

- Configuration: `~/.config/mealplan/config.json`
- Meal Plan (JSON): `~/.config/mealplan/meal_plan.json`
- Meal Plan (Markdown): `~/.config/mealplan/meal_plan.md`

## Development

### Running Tests

```bash
cargo test
```

### Building Documentation

```bash
cargo doc --open
```

## License

This project is licensed under the Apache License, Version 2.0 - see the LICENSE file for details.
