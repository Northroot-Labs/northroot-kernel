//! Get command implementation.

use northroot_canonical::{Digest, DigestAlg};
use northroot_store::{EventIdFilter, FilteredReader, JournalBackendReader, ReadMode, StoreReader};
use crate::output;

pub fn run(journal: String, event_id: String) -> Result<(), Box<dyn std::error::Error>> {
    let reader = JournalBackendReader::open(&journal, ReadMode::Strict)
        .map_err(|e| format!("Failed to open journal: {}", e))?;

    // Parse event ID
    let digest = Digest::new(DigestAlg::Sha256, event_id)
        .map_err(|e| format!("Invalid event ID: {}", e))?;

    // Create filtered reader
    let filter = EventIdFilter { event_id: digest };
    let mut filtered = FilteredReader::new(reader, filter);

    // Find the event
    match filtered.read_next()? {
        Some(event) => {
            println!("{}", output::format_json(&event));
            Ok(())
        }
        None => {
            eprintln!("Event not found");
            std::process::exit(1);
        }
    }
}

