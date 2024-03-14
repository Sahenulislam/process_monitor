use serde::{Deserialize, Serialize};
use std::{
    env, fs,
    sync::{Arc, Mutex},
    thread,
    time::{Duration, SystemTime},
};
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
    #[serde(rename = "type")]
    monitor_type: Option<String>,
    result: Option<Result>,
    code: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Result {
    value: i32,
    processed_at: i64,
}

const TOTAL_DURATION_SECONDS: u64 = 5 * 60;
const UPDATE_INTERVAL_SECONDS: u64 = 30;
const STORE_INTERVAL_SECONDS: u64 = 60;

fn main() {
    let args = env::args().collect::<Vec<String>>();

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


    let monitor_data: MonitorData = match serde_json::from_str(&json_content) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error parsing JSON: {}", err);
            std::process::exit(1);
        }
    };

    let monitor_data = Arc::new(Mutex::new(monitor_data));

    process_monitors(Arc::clone(&monitor_data));
}

fn process_monitors(monitor_data: Arc<Mutex<MonitorData>>) {
   
    let update_handle = {
        let monitor_data = Arc::clone(&monitor_data);
        thread::spawn(move || {
            update_monitors(monitor_data);
        })
    };

  
    let store_handle = {
        let monitor_data = Arc::clone(&monitor_data);
        thread::spawn(move || {
            store_monitors(monitor_data);
        })
    };

  
    update_handle.join().unwrap_or_else(|_| {
        eprintln!("Update thread panicked");
    });
    store_handle.join().unwrap_or_else(|_| {
        eprintln!("Store thread panicked");
    });
}

fn update_monitors(monitor_data: Arc<Mutex<MonitorData>>) {
    let mut elapsed_time = 0;
    while elapsed_time < TOTAL_DURATION_SECONDS {
      
        thread::sleep(Duration::from_secs(UPDATE_INTERVAL_SECONDS));
        let mut data = monitor_data.lock().unwrap();
        for monitor in &mut data.monitors {
            let random_value = rand::thread_rng().gen_range(0..100);
            let current_time = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("SystemTime before UNIX EPOCH!")
                .as_secs();
            monitor.result = Some(Result {
                value: random_value,
                processed_at: current_time as i64,
            });
        }
        elapsed_time += UPDATE_INTERVAL_SECONDS;
    }
}

fn store_monitors(monitor_data: Arc<Mutex<MonitorData>>) {
    let mut elapsed_time = 0;
    while elapsed_time < TOTAL_DURATION_SECONDS {
       
        thread::sleep(Duration::from_secs(STORE_INTERVAL_SECONDS));
        let data = monitor_data.lock().unwrap();
        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("SystemTime before UNIX EPOCH!")
            .as_secs();
        let filename = format!("{}_monitors.json", current_time);
        match serde_json::to_string_pretty(&*data) {
            Ok(json_content) => {
                match fs::write(filename.clone(), json_content) {
                    Ok(_) => println!("Monitors stored in {}", filename),
                    Err(err) => eprintln!("Error storing monitors: {}", err),
                }
            }
            Err(err) => eprintln!("Error converting to JSON: {}", err),
        }
        elapsed_time += STORE_INTERVAL_SECONDS;
    }
}
