# Platform Primitives (Kernel Integration)

This document defines the minimum platform-level primitives that all Northroot-Labs
repos should converge on when they integrate with the trust kernel.

The trust kernel (`northroot-canonical`, `northroot-journal`) remains neutral and
schema-agnostic. These conventions sit above the kernel to reduce org entropy.

## Artifact classes

Two artifact classes must remain distinct:

1) **Operational logs** (mutable, noisy, debugging/telemetry)
2) **Verifiable receipts** (immutable evidence, offline-verifiable)

Operational logs may reference receipts, but logs are not receipts.

## ID taxonomy (v1)

### record_id (operational occurrence identity)

- Use for: JSONL log lines, retries, step attempts, progress updates.
- Format: UUID (v4/v7).
- Not verifiable by itself; do not treat as evidence identity.

### event_id (verifiable receipt identity)

- Use for: Northroot verifiable events / receipts.
- Format: content-derived digest over canonical bytes of the event envelope
  (domain-separated; excludes the `event_id` field to avoid self-reference).
- Offline-verifiable: a verifier recomputes the digest from canonicalization rules.

### digest / blob digest (raw bytes identity)

- Use for: files and artifacts (PDF, CSV, XLSX, JSON, markdown, etc.).
- Format: digest of raw bytes (sha-256 by default).

### content_ref (pointer to external bytes)

- Shape: `{ digest, size_bytes?, media_type? }`
- Use for: binding receipts to real input/output bytes without embedding large blobs.

Normative schema references:
- `schemas/platform/v1/ids.schema.json`
- `schemas/canonical/v1/types.schema.json` (Digest, ContentRef, Timestamp, etc.)

## Boundary guidance: where to emit receipts

Emit verifiable receipts at boundaries that matter for auditability:
- intent accepted
- authorization/gate decision
- execution performed (artifact emitted)
- commit applied (promotion/finalization)

Keep operational logs separate (JSONL is fine). Logs MAY reference receipt `event_id`s.

