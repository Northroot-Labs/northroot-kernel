# Testing Guide

How to write tests, run the QA harness, and add golden tests.

## Quick Start

Run all fast checks before pushing:

```bash
just qa
```

This runs: format check, clippy, tests, and golden tests.

## Test Types

### Unit Tests

Located alongside source code in `src/` with `#[cfg(test)]` modules.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonicalization() {
        // Test implementation
    }
}
```

Run with:
```bash
cargo test
```

### Integration Tests

Located in `tests/` directories at crate root.

```rust
// tests/integration.rs
use northroot_canonical::{compute_event_id, Canonicalizer, ProfileId};
use northroot_journal::{JournalWriter, JournalReader, WriteOptions, ReadMode};
use serde_json::json;

#[test]
fn test_journal_roundtrip() {
    let profile = ProfileId::parse("northroot-canonical-v1").unwrap();
    let canonicalizer = Canonicalizer::new(profile);
    
    // Write event
    let mut event = json!({
        "event_type": "test",
        "event_version": "1",
        "occurred_at": "2024-01-01T00:00:00Z",
        "principal_id": "service:test",
        "canonical_profile_id": "northroot-canonical-v1"
    });
    
    let event_id = compute_event_id(&event, &canonicalizer).unwrap();
    event["event_id"] = serde_json::to_value(&event_id).unwrap();
    
    let mut writer = JournalWriter::open("test.nrj", WriteOptions::default()).unwrap();
    writer.append_event(&event).unwrap();
    writer.finish().unwrap();
    
    // Read event
    let mut reader = JournalReader::open("test.nrj", ReadMode::Strict).unwrap();
    let read_event = reader.read_event().unwrap().unwrap();
    
    assert_eq!(read_event["event_id"], event["event_id"]);
}
```

Run with:
```bash
cargo test --test integration
```

### Golden Tests

Golden tests verify canonicalization stability and hash determinism.

Located in `crates/northroot-canonical/tests/golden.rs`.

Run with:
```bash
just golden
```

To update golden files after intentional changes:
```bash
UPDATE_GOLDEN=1 cargo test --test golden
```

## Running Tests

For information on running tests, CI workflows, and the QA harness, see [QA Harness](../qa/harness.md).

## Writing Tests

### Test Structure

1. **Arrange**: Set up test data and state
2. **Act**: Execute the code under test
3. **Assert**: Verify expected outcomes

### Example: Testing Event Verification

```rust
use northroot_canonical::{compute_event_id, verify_event_id, Canonicalizer, Digest, ProfileId};
use northroot_journal::{JournalWriter, WriteOptions, EventJson};

#[test]
fn test_verify_event_id() {
    // Arrange
    let profile = ProfileId::parse("northroot-canonical-v1").unwrap();
    let canonicalizer = Canonicalizer::new(profile);
    let event = create_test_event();
    
    // Act
    let computed_id = compute_event_id(&event, &canonicalizer).unwrap();
    let is_valid = verify_event_id(&event, &computed_id, &canonicalizer).unwrap();
    
    // Assert
    assert!(is_valid);
}

#[test]
fn test_invalid_event_id() {
    let profile = ProfileId::parse("northroot-canonical-v1").unwrap();
    let canonicalizer = Canonicalizer::new(profile);
    let event = create_test_event();
    let wrong_id = Digest::new(
        northroot_canonical::DigestAlg::Sha256,
        "wrong_digest_value_here"
    ).unwrap();
    
    let is_valid = verify_event_id(&event, &wrong_id, &canonicalizer).unwrap();
    assert!(!is_valid);
}
```

## Property-Based Testing

For critical paths (canonicalization, hashing), consider property-based tests:

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn canonical_bytes_are_deterministic(input in any::<serde_json::Value>()) {
        let c1 = canonicalize(&input)?;
        let c2 = canonicalize(&input)?;
        assert_eq!(c1, c2);
    }
}
```

## Best Practices

1. **Test public APIs**: Focus on public interfaces, not implementation details
2. **Test error cases**: Verify error handling and edge cases
3. **Keep tests fast**: Unit tests should run in milliseconds
4. **Use descriptive names**: Test names should describe what they verify
5. **Avoid test interdependencies**: Tests should be independent and runnable in any order

## Running Tests and CI

For information on running tests, CI workflows, coverage reports, and the QA harness, see [QA Harness](../qa/harness.md).

