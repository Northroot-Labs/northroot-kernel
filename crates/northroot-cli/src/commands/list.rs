//! List command implementation.

use northroot_canonical::Timestamp;
use northroot_store::{
    AndFilter, EventFilter, EventTypeFilter, FilteredReader, PrincipalFilter,
    ReadMode, StoreReader, TimeRangeFilter, JournalBackendReader,
};
use crate::output;

pub fn run(
    journal: String,
    event_type: Option<String>,
    principal: Option<String>,
    after: Option<String>,
    before: Option<String>,
    json: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let reader = JournalBackendReader::open(&journal, ReadMode::Strict)
        .map_err(|e| format!("Failed to open journal: {}", e))?;

    // Build composite filter
    let mut filters: Vec<Box<dyn EventFilter>> = Vec::new();

    if let Some(et) = event_type {
        filters.push(Box::new(EventTypeFilter { event_type: et }));
    }

    if let Some(pid) = principal {
        filters.push(Box::new(PrincipalFilter {
            principal_id: pid,
        }));
    }

    if after.is_some() || before.is_some() {
        let after_ts = after
            .map(|s| Timestamp::parse(s).map_err(|e| format!("Invalid 'after' timestamp: {}", e)))
            .transpose()?;
        let before_ts = before
            .map(|s| Timestamp::parse(s).map_err(|e| format!("Invalid 'before' timestamp: {}", e)))
            .transpose()?;

        filters.push(Box::new(TimeRangeFilter {
            after: after_ts,
            before: before_ts,
        }));
    }

    // Output header if table format
    if !json {
        output::print_table_header();
    }

    // Apply filters and iterate
    if filters.is_empty() {
        // No filters, use reader directly
        let mut reader = reader;
        while let Some(event) = reader.read_next()? {
            if json {
                println!("{}", serde_json::to_string(&event)?);
            } else {
                println!("{}", output::format_table_row(&event));
            }
        }
    } else {
        // Apply filters - combine with AND
        let and_filter = AndFilter { filters };
        let mut filtered = FilteredReader::new(reader, and_filter);
        while let Some(event) = filtered.read_next()? {
            if json {
                println!("{}", serde_json::to_string(&event)?);
            } else {
                println!("{}", output::format_table_row(&event));
            }
        }
    }

    Ok(())
}

