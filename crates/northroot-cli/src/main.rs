//! Northroot CLI - Command-line interface for event verification and journal operations.

use clap::{Parser, Subcommand};

mod commands;
mod output;
mod path;

use commands::{canonicalize, checkpoint, list, verify};

#[derive(Parser)]
#[command(name = "northroot")]
#[command(about = "Northroot event verification and journal operations CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List events in a journal
    List {
        /// Path to journal file
        journal: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
        /// Stop after reading N events (default: unlimited)
        #[arg(long)]
        max_events: Option<u64>,
        /// Reject journals larger than SIZE bytes (default: unlimited)
        #[arg(long)]
        max_size: Option<u64>,
    },
    /// Verify all event IDs in a journal
    Verify {
        /// Path to journal file
        journal: String,
        /// Exit with error code if any verification fails
        #[arg(long)]
        strict: bool,
        /// Output as JSON
        #[arg(long)]
        json: bool,
        /// Stop after reading N events (default: unlimited)
        #[arg(long)]
        max_events: Option<u64>,
        /// Reject journals larger than SIZE bytes (default: unlimited)
        #[arg(long)]
        max_size: Option<u64>,
    },
    /// Show canonical bytes for input JSON
    Canonicalize {
        /// Input JSON file (or stdin if not provided)
        input: Option<String>,
    },
    /// Create a checkpoint event for a journal
    Checkpoint {
        /// Path to journal file
        journal: String,
        /// Principal ID
        #[arg(long)]
        principal: String,
        /// Output checkpoint event as JSON
        #[arg(long)]
        json: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::List {
            journal,
            json,
            max_events,
            max_size,
        } => list::run(journal, json, max_events, max_size),
        Commands::Verify {
            journal,
            strict,
            json,
            max_events,
            max_size,
        } => verify::run(journal, strict, json, max_events, max_size),
        Commands::Canonicalize { input } => canonicalize::run(input),
        Commands::Checkpoint {
            journal,
            principal,
            json,
        } => checkpoint::run(journal, principal, json),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
