# Process Monitor

Process Monitor is a Rust command-line tool for generating random results for monitoring processes. It reads monitor data from a JSON file, adds random results for each monitor, and outputs the updated JSON data.

## Features

- Reads monitor data from a JSON file.
- Generates random results for each monitor.
- Outputs the updated JSON data.

## Installation

To use Process Monitor, you need to have Rust installed on your system. You can install Rust from [https://www.rust-lang.org/](https://www.rust-lang.org/).

Once Rust is installed, you can clone this repository and build the project using Cargo, Rust's package manager and build system:

```bash
git clone https://github.com/yourusername/your-repository.git
cd process_monitor
cargo build --release