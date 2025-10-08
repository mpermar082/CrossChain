// src/main.rs
/*
 * Main executable for CrossChain
 */

use clap::Parser;
use crosschain::{Result, run};

/// CLI arguments for the CrossChain application
#[derive(Parser)]
#[command(version, about = "CrossChain - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Path to input file
    #[arg(short, long, default_value = "input.txt")]
    input: Option<String>,

    /// Path to output file
    #[arg(short, long, default_value = "output.txt")]
    output: Option<String>,
}

fn main() -> Result<()> {
    // Parse CLI arguments
    let args = Cli::parse();

    // Run the application with the provided arguments
    run(args.verbose, args.input, args.output)
}