use northroot_canonical::{compute_event_id, Canonicalizer, ProfileId};
use northroot_journal::verify_event_id;
use serde_json::json;

fn make_canonicalizer() -> Canonicalizer {
    Canonicalizer::new(ProfileId::parse("northroot-canonical-v1").unwrap())
}

fn make_test_event() -> serde_json::Value {
    // Create a minimal valid event
    let mut event = json!({
        "event_type": "test",
        "event_version": "1",
        "occurred_at": "2024-01-01T00:00:00Z",
        "principal_id": "service:test",
        "canonical_profile_id": "northroot-canonical-v1",
        "data": "test data"
    });

    // Compute and set event_id
    let canonicalizer = make_canonicalizer();
    let event_id = compute_event_id(&event, &canonicalizer).unwrap();
    event["event_id"] = json!({
        "alg": "sha-256",
        "b64": event_id.b64
    });

    event
}

#[test]
fn test_verify_event_id_valid() {
    let canonicalizer = make_canonicalizer();
    let event = make_test_event();

    let valid = verify_event_id(&event, &canonicalizer).unwrap();
    assert!(valid);
}

#[test]
fn test_verify_event_id_invalid() {
    let canonicalizer = make_canonicalizer();
    let mut event = make_test_event();

    // Tamper with event_id
    event["event_id"]["b64"] = json!("tampered");

    let valid = verify_event_id(&event, &canonicalizer).unwrap();
    assert!(!valid);
}
