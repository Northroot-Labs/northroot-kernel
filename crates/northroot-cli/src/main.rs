//! Northroot CLI - Command-line interface for event storage and verification.

use clap::{Parser, Subcommand};

mod commands;
mod output;

use commands::{gen, get, inspect, list, verify};

#[derive(Parser)]
#[command(name = "northroot")]
#[command(about = "Northroot event storage and verification CLI")]
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
        /// Filter by event type
        #[arg(long)]
        r#type: Option<String>,
        /// Filter by principal ID
        #[arg(long)]
        principal: Option<String>,
        /// Filter events after this timestamp (RFC3339)
        #[arg(long)]
        after: Option<String>,
        /// Filter events before this timestamp (RFC3339)
        #[arg(long)]
        before: Option<String>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Get a single event by ID
    Get {
        /// Path to journal file
        journal: String,
        /// Event ID (base64url digest)
        event_id: String,
    },
    /// Verify all events in a journal
    Verify {
        /// Path to journal file
        journal: String,
        /// Exit with error code if any verification fails
        #[arg(long)]
        strict: bool,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Inspect authorization and linked executions
    Inspect {
        /// Path to journal file
        journal: String,
        /// Authorization event ID
        #[arg(long)]
        auth: String,
    },
    /// Generate a test journal with deterministic events
    Gen {
        /// Output journal path
        #[arg(long, short)]
        output: String,
        /// Seed for deterministic ID generation (default: 0)
        #[arg(long, default_value = "0")]
        seed: u64,
        /// Number of auth events
        #[arg(long, default_value = "2")]
        count_auth: u32,
        /// Number of valid execution events (one per auth)
        #[arg(long, default_value = "2")]
        count_exec_ok: u32,
        /// Number of deny/orphan execution events
        #[arg(long, default_value = "0")]
        count_exec_bad: u32,
        /// Start timestamp (RFC3339)
        #[arg(long, default_value = "2024-01-01T00:00:00Z")]
        start_ts: String,
        /// Timestamp step in milliseconds
        #[arg(long, default_value = "1000")]
        ts_step_ms: u64,
        /// Include one malformed record for error testing
        #[arg(long)]
        with_bad: bool,
        /// Overwrite existing file
        #[arg(long)]
        force: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::List {
            journal,
            r#type,
            principal,
            after,
            before,
            json,
        } => list::run(journal, r#type, principal, after, before, json),
        Commands::Get { journal, event_id } => get::run(journal, event_id),
        Commands::Verify {
            journal,
            strict,
            json,
        } => verify::run(journal, strict, json),
        Commands::Inspect { journal, auth } => inspect::run(journal, auth),
        Commands::Gen {
            output,
            seed,
            count_auth,
            count_exec_ok,
            count_exec_bad,
            start_ts,
            ts_step_ms,
            with_bad,
            force,
        } => gen::run(
            output, seed, count_auth, count_exec_ok, count_exec_bad, start_ts, ts_step_ms,
            with_bad, force,
        ),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

