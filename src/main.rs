mod batch;
mod config;
mod report;
mod scanner;

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The path to the folder to scan
    #[arg(short, long, value_name = "FOLDER_PATH")]
    path: PathBuf,

    /// Optional: The name of a specific Pioneer DJ device to check compatibility against
    #[arg(short, long, value_name = "DEVICE_NAME")]
    device: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let all_devices = config::get_devices();
    
    let devices_to_scan = if let Some(device_name) = cli.device {
        if let Some(device) = all_devices.iter().find(|d| d.name.eq_ignore_ascii_case(&device_name)) {
            vec![device.clone()]
        } else {
            eprintln!("Error: Device '{}' not found.", device_name);
            eprintln!("Available devices:");
            for device in all_devices {
                eprintln!("- {}", device.name);
            }
            return;
        }
    } else {
        all_devices
    };

    let results = batch::batch_scan(&cli.path, &devices_to_scan);
    let report = report::generate_report(&results);
    println!("{}", report);
}
