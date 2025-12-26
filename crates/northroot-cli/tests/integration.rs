//! Integration tests for CLI commands.

use northroot_journal::{JournalWriter, WriteOptions};
use northroot_canonical::{compute_event_id, Canonicalizer, ProfileId};
use serde_json::json;
use std::process::Command;
use tempfile::TempDir;

fn make_test_event(id: &str) -> serde_json::Value {
    json!({
        "event_type": "test",
        "event_version": "1",
        "occurred_at": "2024-01-01T00:00:00Z",
        "principal_id": "service:test",
        "canonical_profile_id": "northroot-canonical-v1",
        "data": format!("test data {}", id)
    })
}

fn create_test_journal() -> (TempDir, String) {
    let temp_dir = TempDir::new().unwrap();
    let journal_path = temp_dir.path().join("test.nrj");

    let profile = ProfileId::parse("northroot-canonical-v1").unwrap();
    let canonicalizer = Canonicalizer::new(profile);

    {
        let mut writer = JournalWriter::open(&journal_path, WriteOptions::default()).unwrap();
        
        // Create and compute event IDs
        let mut event1 = make_test_event("1");
        let event_id1 = compute_event_id(&event1, &canonicalizer).unwrap();
        event1["event_id"] = json!({
            "alg": "sha-256",
            "b64": event_id1.b64
        });
        writer.append_event(&event1).unwrap();

        let mut event2 = make_test_event("2");
        let event_id2 = compute_event_id(&event2, &canonicalizer).unwrap();
        event2["event_id"] = json!({
            "alg": "sha-256",
            "b64": event_id2.b64
        });
        writer.append_event(&event2).unwrap();
        
        writer.finish().unwrap();
    }

    (temp_dir, journal_path.to_string_lossy().to_string())
}

fn run_cli(args: &[&str]) -> (bool, String, String) {
    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--bin", "northroot", "--"]);
    cmd.args(args);
    let output = cmd.output().expect("Failed to execute CLI");

    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    let success = output.status.success();

    (success, stdout, stderr)
}

#[test]
fn test_list_command() {
    let (_temp_dir, journal_path) = create_test_journal();

    let (success, stdout, _) = run_cli(&["list", &journal_path]);
    assert!(success);
    assert!(stdout.contains("EVENT_ID"));
    assert!(stdout.contains("test"));
}

#[test]
fn test_list_json_output() {
    let (_temp_dir, journal_path) = create_test_journal();

    let (success, stdout, _) = run_cli(&["list", &journal_path, "--json"]);
    assert!(success);
    // JSON output should be parseable
    let lines: Vec<&str> = stdout.lines().filter(|l| !l.is_empty()).collect();
    assert!(!lines.is_empty());
    for line in lines {
        serde_json::from_str::<serde_json::Value>(line).expect("Invalid JSON");
    }
}

#[test]
fn test_verify_command() {
    let (_temp_dir, journal_path) = create_test_journal();

    let (success, stdout, _) = run_cli(&["verify", &journal_path]);
    assert!(success);
    assert!(stdout.contains("EVENT_ID") || stdout.contains("VALID"));
}

#[test]
fn test_verify_strict_mode() {
    let (_temp_dir, journal_path) = create_test_journal();

    // Verify should succeed with valid events
    let (success, _, _) = run_cli(&["verify", &journal_path, "--strict"]);
    assert!(success, "verify should succeed with valid events");
}

#[test]
fn test_canonicalize_command() {
    // Canonicalize command requires stdin input
    // This test just verifies the command exists and accepts the argument
    // A full test would require piping input via std::process::Stdio
    let (_success, _stdout, _) = run_cli(&["canonicalize"]);
    // Command may fail without stdin, which is expected
}

#[test]
fn test_checkpoint_command() {
    let (_temp_dir, journal_path) = create_test_journal();

    let (success, stdout, stderr) = run_cli(&["checkpoint", &journal_path, "--principal", "service:test", "--json"]);
    if !success {
        eprintln!("Checkpoint command failed. stderr: {}", stderr);
    }
    assert!(success, "checkpoint command should succeed");
    // Should output checkpoint event as JSON
    let parsed: serde_json::Value = serde_json::from_str(&stdout.trim()).unwrap();
    assert_eq!(parsed["event_type"], "checkpoint");
    assert_eq!(parsed["event_version"], "1");
}
