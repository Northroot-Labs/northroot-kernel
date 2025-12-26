//! Append command implementation.

use crate::path;
use northroot_canonical::{compute_event_id, Canonicalizer, ProfileId};
use northroot_journal::{JournalWriter, WriteOptions};
use serde_json::Value;
use std::io::{self, Read};

pub fn run(
    journal: String,
    input: Option<String>,
    strict: bool,
    sync: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // Validate journal path (allow non-existent files for creation)
    let journal_path = if std::path::Path::new(&journal).exists() {
        path::validate_journal_path(&journal, false)
            .map_err(|e| format!("Invalid journal path: {}", e))?
    } else {
        // For new files, validate the parent directory exists and path is safe
        let path = std::path::Path::new(&journal);
        let parent = path.parent()
            .ok_or_else(|| format!("Invalid journal path: {}", journal))?;
        
        // Resolve parent directory to check it exists
        let parent_abs = if parent.is_absolute() {
            parent.to_path_buf()
        } else {
            std::env::current_dir()?
                .join(parent)
                .canonicalize()
                .map_err(|e| format!("Invalid journal path: {}: {}", journal, e))?
        };
        
        // Check for path traversal in the resolved path
        let path_str = parent_abs.to_string_lossy();
        if path_str.contains("..") {
            return Err(format!("Invalid journal path: path contains traversal sequences: {}", journal).into());
        }
        
        // Return the absolute path for the journal file
        if path.is_absolute() {
            path.to_path_buf()
        } else {
            std::env::current_dir()?.join(path)
        }
    };

    // Read JSON from file or stdin
    let json_str = if let Some(path) = input {
        std::fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read file {}: {}", path, e))?
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    };

    let mut event: Value = serde_json::from_str(&json_str)
        .map_err(|e| format!("Invalid JSON: {}", e))?;

    // Initialize canonicalizer
    let profile = ProfileId::parse("northroot-canonical-v1")
        .map_err(|e| format!("Invalid profile ID: {}", e))?;
    let canonicalizer = Canonicalizer::new(profile);

    // If strict mode, check existing event_id before computing
    if strict {
        if let Some(existing_id) = event.get("event_id") {
            // Compute event_id and compare
            let computed_id = compute_event_id(&event, &canonicalizer)
                .map_err(|e| format!("Event ID computation failed: {}", e))?;
            
            let existing_id_str = serde_json::to_string(existing_id)?;
            let computed_id_str = serde_json::to_string(&computed_id)?;
            
            if existing_id_str != computed_id_str {
                return Err(format!(
                    "Event ID mismatch: computed {} but event has {}",
                    computed_id_str, existing_id_str
                )
                .into());
            }
            // Event ID matches, use the existing one
        }
    }

    // Compute event_id (will be used if not already present or not in strict mode)
    let event_id = compute_event_id(&event, &canonicalizer)
        .map_err(|e| format!("Event ID computation failed: {}", e))?;

    // Add event_id to event (overwrites if already present, which is fine)
    event["event_id"] = serde_json::to_value(&event_id)?;

    // Open journal for writing
    let write_options = WriteOptions {
        sync,
        create: true,
        append: true,
    };

    let mut writer = JournalWriter::open(&journal_path, write_options).map_err(|e| {
        let sanitized = path::sanitize_path_for_error(&journal_path);
        format!("Failed to open journal file: {}: {}", sanitized, e)
    })?;

    // Append event
    writer.append_event(&event).map_err(|e| {
        let sanitized = path::sanitize_path_for_error(&journal_path);
        format!("Failed to append event to journal: {}: {}", sanitized, e)
    })?;

    // Finish writing (closes file)
    writer.finish().map_err(|e| {
        let sanitized = path::sanitize_path_for_error(&journal_path);
        format!("Failed to finish writing journal: {}: {}", sanitized, e)
    })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use northroot_journal::JournalReader;
    use serde_json::json;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_append_event_to_new_journal() {
        let temp = TempDir::new().unwrap();
        std::env::set_current_dir(temp.path()).unwrap();
        let journal_path = temp.path().join("test.nrj");
        let journal_str = journal_path.to_str().unwrap(); // Use absolute path

        let event = json!({
            "event_type": "test",
            "event_version": "1",
            "occurred_at": "2024-01-01T00:00:00Z",
            "principal_id": "service:test",
            "canonical_profile_id": "northroot-canonical-v1"
        });

        // Test with file input
        let event_file = temp.path().join("event.json");
        fs::write(&event_file, serde_json::to_string(&event).unwrap()).unwrap();

        let result = run(
            journal_str.to_string(),
            Some(event_file.to_str().unwrap().to_string()),
            false,
            false,
        );
        assert!(result.is_ok(), "Append failed: {:?}", result.err());

        // Verify event was appended
        let mut reader = JournalReader::open(&journal_path, northroot_journal::ReadMode::Strict).unwrap();
        let read_event = reader.read_event().unwrap().unwrap();
        assert_eq!(read_event["event_type"], "test");
        assert!(read_event.get("event_id").is_some());
    }

    #[test]
    fn test_append_multiple_events() {
        let temp = TempDir::new().unwrap();
        std::env::set_current_dir(temp.path()).unwrap();
        let journal_path = temp.path().join("test.nrj");
        let journal_str = journal_path.to_str().unwrap(); // Use absolute path

        let event1 = json!({
            "event_type": "test1",
            "event_version": "1",
            "occurred_at": "2024-01-01T00:00:00Z",
            "principal_id": "service:test",
            "canonical_profile_id": "northroot-canonical-v1"
        });

        let event2 = json!({
            "event_type": "test2",
            "event_version": "1",
            "occurred_at": "2024-01-01T00:01:00Z",
            "principal_id": "service:test",
            "canonical_profile_id": "northroot-canonical-v1"
        });

        // Append first event
        let event_file1 = temp.path().join("event1.json");
        fs::write(&event_file1, serde_json::to_string(&event1).unwrap()).unwrap();
        run(
            journal_str.to_string(),
            Some(event_file1.to_str().unwrap().to_string()),
            false,
            false,
        ).unwrap();

        // Append second event
        let event_file2 = temp.path().join("event2.json");
        fs::write(&event_file2, serde_json::to_string(&event2).unwrap()).unwrap();
        run(
            journal_str.to_string(),
            Some(event_file2.to_str().unwrap().to_string()),
            false,
            false,
        ).unwrap();

        // Verify both events
        let mut reader = JournalReader::open(&journal_path, northroot_journal::ReadMode::Strict).unwrap();
        let read_event1 = reader.read_event().unwrap().unwrap();
        assert_eq!(read_event1["event_type"], "test1");

        let read_event2 = reader.read_event().unwrap().unwrap();
        assert_eq!(read_event2["event_type"], "test2");
    }

    #[test]
    fn test_append_invalid_json() {
        let temp = TempDir::new().unwrap();
        std::env::set_current_dir(temp.path()).unwrap();
        let journal_path = temp.path().join("test.nrj");
        let journal_str = journal_path.to_str().unwrap(); // Use absolute path

        let invalid_file = temp.path().join("invalid.json");
        fs::write(&invalid_file, "{ invalid json }").unwrap();

        let result = run(
            journal_str.to_string(),
            Some(invalid_file.to_str().unwrap().to_string()),
            false,
            false,
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid JSON"));
    }

    #[test]
    fn test_append_strict_mode_with_mismatched_event_id() {
        let temp = TempDir::new().unwrap();
        std::env::set_current_dir(temp.path()).unwrap();
        let journal_path = temp.path().join("test.nrj");
        let journal_str = journal_path.to_str().unwrap(); // Use absolute path

        // Create event with mismatched event_id
        let event = json!({
            "event_type": "test",
            "event_version": "1",
            "occurred_at": "2024-01-01T00:00:00Z",
            "principal_id": "service:test",
            "canonical_profile_id": "northroot-canonical-v1",
            "event_id": {"alg": "sha-256", "b64": "wrong_id"}
        });

        let event_file = temp.path().join("event.json");
        fs::write(&event_file, serde_json::to_string(&event).unwrap()).unwrap();

        let result = run(
            journal_str.to_string(),
            Some(event_file.to_str().unwrap().to_string()),
            true, // strict mode
            false,
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Event ID mismatch"));
    }
}

