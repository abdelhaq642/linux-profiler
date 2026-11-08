# ADR 0001: Shared domain and crate dependency direction

Status: Accepted

## Context

The Linux Performance Profiler targets Linux x86_64 and aarch64. Capture,
processing, and presentation must remain separate, following
[the AI contract](../ai_contract.md).

## Decision

Use a virtual Cargo workspace with nine crates under `crates/`:

- `lpp-cli`: command-line entry point with placeholder `record`, `report`, `serve`.
- `lpp-core`: shared domain types only; addresses use `u64`.
- `lpp-perf`: future perf event capture.
- `lpp-unwind`: future stack unwinding.
- `lpp-symbolize`: future symbol resolution.
- `lpp-bpf`: future eBPF integration.
- `lpp-store`: future persistence.
- `lpp-analysis`: future profile analysis.
- `lpp-server`: future serving API.

The initial graph is deliberately minimal: every other crate depends on
`lpp-core`; `lpp-core` depends on no crates. All eight edges point inward.
There are no other inter-crate edges or third-party dependencies yet.
Do not add speculative service wiring or implementations in this scaffold.

`lpp-core` must not acquire platform bindings, I/O, storage, capture, or
orchestration responsibilities. Higher-level composition belongs outside core.
Future orchestration can depend on the relevant boundary crates, never the
reverse, and must keep the graph acyclic.

Symbolization and database writes must remain off sampling threads. Future
capture queues must be bounded, lost events surfaced, and architecture-specific
behavior isolated. The scaffold implements none of these mechanisms yet.

## Consequences

Domain types can be shared without bringing kernel or native dependencies into
consumers. All crates currently forbid unsafe code via workspace lints; future
low-level integration needs an explicit, narrowly scoped policy adjustment and
SAFETY comments for every unsafe block. No production commands are shelled out.

The old root `src/` is retained untouched to preserve existing local work. As
this is now a virtual workspace, those files are not compiled or used by the CLI.
