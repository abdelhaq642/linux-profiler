//! Placeholder CLI; no profiling, reporting, or serving is implemented.

use std::process::ExitCode;

const USAGE: &str = "Usage: lpp-cli <record|report|serve>";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.as_slice() {
        [command] if matches!(command.as_str(), "record" | "report" | "serve") => {
            println!("{command}: not implemented yet");
            ExitCode::SUCCESS
        }
        [flag] if matches!(flag.as_str(), "--help" | "-h") => {
            println!("{USAGE}");
            ExitCode::SUCCESS
        }
        _ => {
            eprintln!("{USAGE}");
            ExitCode::from(2)
        }
    }
}
