use northroot_canonical::{canonicalizer::Canonicalizer, ProfileId};
use serde_json::json;

fn main() {
    let profile = ProfileId::parse("profileid000000001").expect("valid profile");
    let canonicalizer = Canonicalizer::new(profile);
    let event = json!({
        "event_type": "example",
        "event_version": "1",
        "occurred_at": "2025-12-20T00:00:00Z",
        "principal_id": "human:alice",
        "canonical_profile_id": "profileid000000001",
        "payload": {
            "value": 42
        }
    });

    match canonicalizer.canonicalize(&event) {
        Ok(result) => {
            println!("{}", String::from_utf8_lossy(&result.bytes));
        }
        Err(err) => {
            eprintln!("canonicalization failed: {}", err);
            std::process::exit(1);
        }
    }
}
