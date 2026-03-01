use crate::cli::run_cli;

mod cli;
mod config;
mod error;
mod manifest;
mod template;

fn main() {
    if let Err(e) = run_cli() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
