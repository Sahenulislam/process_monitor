use serde::{Deserialize, Serialize};
use std::{env, fs, time::SystemTime};
use rand::Rng;

#[derive(Debug, Deserialize, Serialize)]
struct MonitorData {
    monitors: Vec<Monitor>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Monitor {
    monitor_id: Option<i32>,
    name: String,
    script: Option<String>,
    _type: Option<String>,
    result: Option<Result>,
    code: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Result {
    value: i32,
    processed_at: i64,
}

fn main() {
    
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <json-file-path>", args[0]);
        std::process::exit(1);
    }
    
    let file_path = &args[2];

    let json_content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            std::process::exit(1);
        }
    };

    println!("----------------Initial JSON----------------");
    println!("{}", json_content);
    
    let mut monitor_data: MonitorData = match serde_json::from_str(&json_content) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error parsing JSON: {}", err);
            std::process::exit(1);
        }
    };

    for monitor in &mut monitor_data.monitors {
        let random_value = rand::thread_rng().gen::<i32>();
        
        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("SystemTime before UNIX EPOCH!")
            .as_secs();
        
        monitor.result = Some(Result {
            value: random_value,
            processed_at: current_time as i64,
        });
    }

    let updated_json_content = match serde_json::to_string_pretty(&monitor_data) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error converting to JSON: {}", err);
            std::process::exit(1);
        }
    };
    println!("----------------Modified JSON----------------");
    println!("{}", updated_json_content);
}
