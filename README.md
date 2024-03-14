# Process Monitor

Process Monitor is a Rust command-line tool for generating random results for monitoring processes. It reads monitor data from a JSON file, adds random results for each monitor, and outputs the updated JSON data.

## Features

-Flexible Configuration: Allows users to specify monitors in a JSON file, including name, script path (if applicable), monitor type, and unique code.
-Real-time Monitoring: Utilizes threads to update monitor results at defined intervals.
-Data Storage: Periodically stores monitor data in JSON files with timestamps for future analysis.
-Concurrency: Utilizes Arc and Mutex for safe concurrent access to shared data structures.

## Installation

To use Process Monitor, you need to have Rust installed on your system. You can install Rust from [https://www.rust-lang.org/](https://www.rust-lang.org/).

Once Rust is installed, you can clone this repository and build the project using Cargo, Rust's package manager and build system:

``bash
git clone https://github.com/Sahenulislam/process_monitor.git
cd process_monitor
cargo build --release


## Usage

Run the compiled executable with the path to the JSON file containing monitor data as a command-line argument:

``bash
./target/release/process_monitor -monitorFile path/to/your/monitors.json





