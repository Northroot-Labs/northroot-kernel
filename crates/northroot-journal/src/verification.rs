//! Verification helpers for journal events.

use crate::errors::JournalError;
use crate::event::EventJson;
use northroot_canonical::{compute_event_id, Canonicalizer};

/// Verifies an event JSON against its claimed event_id.
///
/// This parses the event, canonicalizes it, and checks that the computed
/// event_id matches the `event_id` field in the JSON.
pub fn verify_event_id(
    event: &EventJson,
    canonicalizer: &Canonicalizer,
) -> Result<bool, JournalError> {
    // Extract event_id from JSON
    let claimed_id = event
        .get("event_id")
        .and_then(|v| serde_json::from_value::<northroot_canonical::Digest>(v.clone()).ok())
        .ok_or_else(|| JournalError::InvalidJson("missing or invalid event_id".to_string()))?;

    // Compute actual event_id
    let computed_id = compute_event_id(event, canonicalizer)
        .map_err(|e| JournalError::InvalidJson(format!("event ID computation failed: {}", e)))?;

    Ok(claimed_id == computed_id)
}
