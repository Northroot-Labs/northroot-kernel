use std::collections::BTreeMap;

use northroot_canonical::{
    canonicalizer::Canonicalizer, ContentRef, Digest, DigestAlg, HygieneReport, HygieneStatus,
    HygieneWarning, ProfileId, Quantity,
};
use serde_json::json;

#[test]
fn digest_serializes_to_golden_json() {
    let digest = Digest {
        alg: DigestAlg::Sha256,
        b64: "Zm9vYmFy".into(),
    };

    assert_eq!(
        serde_json::to_string(&digest).unwrap(),
        r#"{"alg":"sha-256","b64":"Zm9vYmFy"}"#
    );
}

#[test]
fn quantity_dec_serialization_is_deterministic() {
    let quantity = Quantity::Dec {
        m: "12345".into(),
        s: 2,
    };

    assert_eq!(
        serde_json::to_string(&quantity).unwrap(),
        r#"{"t":"dec","m":"12345","s":2}"#
    );
}

#[test]
fn hygiene_report_matches_expected_shape() {
    let report = HygieneReport {
        status: HygieneStatus::Ok,
        warnings: vec![HygieneWarning::new("DuplicateKeys")],
        metrics: BTreeMap::new(),
        profile_id: ProfileId::new("example_profile_0001".into()),
    };

    let serialized = serde_json::to_value(&report).unwrap();
    let expected = json!({
        "status": "Ok",
        "warnings": ["DuplicateKeys"],
        "metrics": {},
        "profile_id": "example_profile_0001"
    });

    assert_eq!(serialized, expected);
}

#[test]
fn canonicalizer_produces_ordered_bytes() {
    let profile = ProfileId::parse("profileid000000001").unwrap();
    let canonicalizer = Canonicalizer::new(profile);
    let value = json!({"b": 1, "a": {"nested": 2}});
    let result = canonicalizer.canonicalize(&value).unwrap();
    assert_eq!(result.bytes, br#"{"a":{"nested":2},"b":1}"#.to_vec());
    assert_eq!(result.report.status, HygieneStatus::Ok);
}

#[test]
fn content_ref_serialization_includes_digest() {
    let payload = json!({
        "alg": "sha-256",
        "b64": "Zm9v"
    });
    let content_ref = ContentRef {
        digest: Digest {
            alg: DigestAlg::Sha256,
            b64: "Zm9v".into(),
        },
        size_bytes: Some(42),
        media_type: Some("application/json".into()),
    };

    assert_eq!(
        serde_json::to_string(&content_ref).unwrap(),
        format!(
            r#"{{"digest":{},"size_bytes":42,"media_type":"application/json"}}"#,
            payload
        )
    );
}
