use serde::{Deserialize, Serialize};
use std::{env, fs};

// Struct for holding monitor data
#[derive(Debug, Deserialize)]
struct MonitorData {
    monitors: Vec<Monitor>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Monitor {
    name: String,
    script: Option<String>,
    result: Option<Result>,
    code: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Result {
    value: i32,
    processed_at: i64,
}

fn main() {
    // Process command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 || args[1] != "-monitorFile" {
        eprintln!("Usage: {} -monitorFile <json-file-path>", args[0]);
        std::process::exit(1);
    }
    let file_path = &args[2];

    // Read the JSON file
    let json_content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            std::process::exit(1);
        }
    };

    // Parse JSON into monitor data
    let monitor_data: MonitorData = match serde_json::from_str(&json_content) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error parsing JSON: {}", err);
            std::process::exit(1);
        }
    };

    // Print monitor data
    println!("Monitor data:");
    for monitor in &monitor_data.monitors {
        println!("{:?}", monitor);
    }
}
