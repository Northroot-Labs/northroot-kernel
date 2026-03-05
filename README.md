# Northroot Kernel

Trust kernel for deterministic, audit-grade evidence recording.

## What is Northroot?

Northroot provides a minimal trust kernel for recording verifiable events. It standardizes canonicalization, event identity computation, and portable evidence storage—without prescribing domain semantics or policy logic.

**Core capabilities:**
- Deterministic canonicalization (RFC 8785 + Northroot rules)
- Content-derived event identity
- Portable journal format (.nrj)
- Offline verification

**What Northroot does NOT do:**
- Make decisions or optimize outcomes
- Execute actions or orchestrate workflows
- Prescribe domain event schemas
- Evaluate policy or enforce constraints

See [GOVERNANCE.md](GOVERNANCE.md) for the project's foundational principles.

## Kernel Hardening Charter (v1)

This batch freezes the kernel scope and success criteria before any cross-repo expansion.

### Goals
- Keep core primitives deterministic and offline-verifiable.
- Standardize identity semantics for verifiable evidence.
- Keep `.nrj` as a portable append-only audit container.
- Publish minimal contracts that downstream repos can adopt without pulling orchestration into core.

### Non-goals
- No orchestration logic, workflow execution, or policy engines.
- No model, runtime, or budget decisioning logic.
- No domain-specific business semantics in core crates.

### Batch acceptance checkpoints
- `charter_frozen`: goals/non-goals and boundaries are explicit.
- `id_vocab_locked`: `record_id`, `content_id`, `event_id` usage is documented and schema-backed.
- `event_contract_v1`: minimal verifiable economic event contract is versioned.
- `journal_contract_v1`: minimal `.nrj` contract is explicit and testable.
- `golden_gates_green`: deterministic gates (fmt/clippy/golden/schema) are green.
- `agent_role_partition_defined`: eval-bootstrap and implementer responsibilities are separated.

### Contract-spine authority references
- `repos/docs/internal/workspace/CONTRACT_SPINE_DECISION_REGISTER_V1.md`
- `repos/docs/internal/workspace/CONTRACT_SPINE_CI_GATE_ALIGNMENT_V1.md`
- `repos/docs/internal/product/NORTHROOT_KERNEL_AND_ID_SEMANTICS_DIRECTION_NOTE.md`

## Quick Start

### Building

```bash
# Build kernel crates
cargo build --workspace

# Build CLI application
cd apps/northroot && cargo build --release
```

The CLI binary will be at `apps/northroot/target/release/northroot`.

### Using the CLI

```bash
# Canonicalize JSON input
echo '{"b":2,"a":1}' | northroot canonicalize

# Compute event_id for JSON
echo '{"event_type":"test","event_version":"1",...}' | northroot event-id

# List events in a journal
northroot list events.nrj

# Verify all events in a journal
northroot verify events.nrj
```

## Documentation

### For Users
- [Getting Started](docs/user/getting-started.md) - Tutorial and examples
- [Integration Examples](docs/user/integration-examples.md) - Code samples

### For Developers
- [API Contract](docs/developer/api-contract.md) - Public API surface
- [Architecture](docs/developer/architecture.md) - System design
- [Testing Guide](docs/developer/testing.md) - QA harness and test patterns
- [Extensions](docs/reference/extensions.md) - How to extend the kernel

### Reference
- [Core Specification](docs/reference/spec.md) - Protocol specification
- [Journal Format](docs/reference/format.md) - On-disk format
- [Canonicalization](docs/reference/canonicalization.md) - Canonical JSON rules
- [Event Model](docs/reference/events.md) - Event structure

## Project Structure

```
northroot/
├── crates/
│   ├── northroot-canonical/  # Canonicalization + event_id
│   └── northroot-journal/    # .nrj container format
├── apps/
│   └── northroot/            # CLI application
├── fixtures/                  # Golden test vectors
├── schemas/
│   └── canonical/             # Canonical primitive schemas
├── wip/                       # Non-core code (governance, agent-domain, store)
├── docs/                      # Documentation
└── GOVERNANCE.md              # Project constitution
```

## Trust Kernel

The kernel provides:
- **Canonicalization**: RFC 8785 + Northroot hygiene rules
- **Event Identity**: `sha256(domain_separator || canonical_json(event))`
- **Journal Format**: Portable, append-only container (.nrj)

Everything else (typed schemas, domain verification, policy evaluation) is extension.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development guidelines, coding standards, and contribution process.

## License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.
