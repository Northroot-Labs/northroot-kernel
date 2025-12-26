//! Checkpoint command implementation.

use crate::path;
use northroot_canonical::{compute_event_id, Canonicalizer, Digest, ProfileId, Timestamp};
use northroot_journal::{JournalReader, JournalWriter, ReadMode, WriteOptions};
use northroot_schemas::CheckpointEvent;
use serde_json::json;

pub fn run(
    journal: String,
    principal: String,
    json_output: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // Validate and normalize journal path
    let journal_path = path::validate_journal_path(&journal, false)
        .map_err(|e| format!("Invalid journal path: {}", e))?;

    let profile = ProfileId::parse("northroot-canonical-v1")
        .map_err(|e| format!("Invalid profile ID: {}", e))?;
    let canonicalizer = Canonicalizer::new(profile.clone());

    // Read journal to find chain tip
    let mut reader = JournalReader::open(&journal_path, ReadMode::Strict).map_err(|e| {
        let sanitized = path::sanitize_path_for_error(&journal_path);
        format!("Failed to open journal file: {}: {}", sanitized, e)
    })?;

    let mut chain_tip_event_id: Option<Digest> = None;
    let mut chain_tip_height: u64 = 0;

    while let Some(event) = reader.read_event()? {
        chain_tip_height += 1;
        // Extract event_id from the event
        if let Some(event_id_value) = event.get("event_id") {
            if let Ok(digest) = serde_json::from_value::<Digest>(event_id_value.clone()) {
                chain_tip_event_id = Some(digest);
            }
        }
    }

    let chain_tip_event_id = chain_tip_event_id.ok_or_else(|| {
        "Journal is empty; cannot create checkpoint without events".to_string()
    })?;

    // Create checkpoint event
    // Use current time in RFC3339 format
    let now_utc = chrono::Utc::now();
    let occurred_at = Timestamp::parse(&format!("{}Z", now_utc.format("%Y-%m-%dT%H:%M:%S")))
        .map_err(|e| format!("Failed to create timestamp: {}", e))?;

    let chain_tip_event_id_clone = chain_tip_event_id.clone();
    let principal_id = northroot_canonical::PrincipalId::parse(&principal)
        .map_err(|e| format!("Invalid principal ID: {}", e))?;
    
    // Create checkpoint event as JSON first (without event_id)
    // Serialize the types to get their string representations
    let occurred_at_str = serde_json::to_string(&occurred_at).unwrap().trim_matches('"').to_string();
    let profile_str = serde_json::to_string(&profile).unwrap().trim_matches('"').to_string();
    
    let checkpoint_json = json!({
        "event_type": "checkpoint",
        "event_version": "1",
        "occurred_at": occurred_at_str,
        "principal_id": principal,
        "canonical_profile_id": profile_str,
        "chain_tip_event_id": {
            "alg": "sha-256",
            "b64": chain_tip_event_id_clone.b64
        },
        "chain_tip_height": chain_tip_height
    });

    // Compute event_id from JSON
    let computed_event_id = compute_event_id(&checkpoint_json, &canonicalizer)?;
    
    // Now create the full checkpoint event struct
    let checkpoint = CheckpointEvent {
        event_id: computed_event_id,
        event_type: "checkpoint".to_string(),
        event_version: "1".to_string(),
        prev_event_id: None,
        occurred_at,
        principal_id,
        canonical_profile_id: profile.clone(),
        chain_tip_event_id: chain_tip_event_id_clone.clone(),
        chain_tip_height,
        merkle_root: None,
        window: None,
    };

    if json_output {
        println!("{}", serde_json::to_string_pretty(&checkpoint)?);
    } else {
        // Write to journal
        let mut writer = JournalWriter::open(&journal_path, WriteOptions::default())
            .map_err(|e| format!("Failed to open journal for writing: {}", e))?;

        let event_json = serde_json::to_value(&checkpoint)?;
        writer.append_event(&event_json)?;
        writer.finish()?;

        println!("Checkpoint created: {}", checkpoint.event_id.b64);
        println!("Chain tip: {}", chain_tip_event_id_clone.b64);
        println!("Chain height: {}", chain_tip_height);
    }

    Ok(())
}

