# Agent-Domain Code (Archived for Future Work)

This directory contains agent-domain-specific code that was removed from the Northroot 1.0 release.

## Contents

- `events.rs` - AuthorizationEvent, ExecutionEvent, and related types
- `verification.rs` - Verifier with bounds checking, USD conversion, and domain-specific verification logic
- `shared.rs` - Meter, ResourceRef, IntentAnchors types
- `errors.rs` - CoreError types

## Status

This code is archived for potential future use in a `northroot-agent` or `northroot-domain` crate. It is not part of the 1.0 release, which focuses on domain-agnostic primitives.

## Notes

- These types are specific to agent orchestration and authorization domains
- They should not be in the neutral trust kernel
- Future work may create a separate crate for agent-domain schemas and verification

