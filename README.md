# get_flutter_cupertino_icons

This Rust project `get_flutter_cupertino_icons` is designed to scrape and extract Cupertino icon data from the official Flutter documentation website. It fetches icon names and their corresponding hexadecimal codes, then saves this information into a `icons.json` file.

## Features

- Scrapes icon data from `https://api.flutter.dev/flutter/cupertino/CupertinoIcons-class.html`.
- Extracts icon names and their hexadecimal codes.
- Saves the collected data as a pretty-printed JSON file (`icons.json`).

## Usage

To run this project, you will need to have Rust and Cargo installed.

1. **Clone the repository:**
   ```bash
   git clone <repository_url>
   cd get_flutter_cupertino_icons
   ```

2. **Run the application:**
   ```bash
   cargo run
   ```

Upon successful execution, a file named `icons.json` will be created in the project's root directory, containing the scraped Cupertino icon data.

## Project Structure

- `src/main.rs`: Contains the core logic for web scraping, data extraction, and JSON serialization.
- `Cargo.toml`: Defines project dependencies and metadata.
- `icons.json`: (Generated) The output file containing the extracted icon data.

## Dependencies

This project relies on the following Rust crates:

- `reqwest`: For making HTTP requests.
- `select`: For parsing HTML documents.
- `serde` and `serde_json`: For serializing and deserializing data to/from JSON.
- `tokio`: An asynchronous runtime for Rust.
- `futures`: For asynchronous programming utilities.
- `regex`: For regular expression matching.
- `once_cell`: For one-time initialization of static variables.