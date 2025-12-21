## Core Model

Northroot records **verifiable evidence** as canonical JSON events. Each event schema (authorization, execution, checkpoint, attestation, etc.) describes every field that belongs to the canonical payload—`event_type`, `event_version`, timestamps, principals, canonicalization profile IDs, and the domain-specific data that policies or auditors care about.

Identity is deterministic: `event_id = H(domain_separator || canonical_json(event))`, where `canonical_json(event)` follows the Northroot canonicalization profile. There is no secondary `v` envelope; the schema-defined object itself is what gets hashed, so every field that influences verification must be exposed there.

Operational metadata (request IDs, trace headers, retries, provider hints, tags, etc.) is handled outside the canonical object. Transport layers may wrap the event or carry extra metadata, but nothing outside the schema should affect the bytes used for `event_id`.

`event_version` captures schema evolution so readers can apply the right canonicalization rules without needing another envelope layer. Attestation events make room for multiple signatures via the `signatures` array (1–16 entries) so multiple trust anchors can independently attest to the same checkpoint.

