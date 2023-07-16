use crate::interface::args::Cli;
use clap::Parser;

pub fn run_rusty() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse(); // Parse command-line arguments

    println!("{}", cli.channel);
    println!("Hello from runnner prolly");

    Ok(()) // Return success
}
