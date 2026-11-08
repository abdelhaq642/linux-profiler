# AI implementation contract

## Architecture
- Linux only.
- Support x86_64 and aarch64.
- Rust userspace; C only for eBPF and narrow native FFI shims.
- Never shell out to perf, bpftrace, addr2line, or FlameGraph
  in production code.
- Direct perf_event_open integration is mandatory.
- eBPF production code uses libbpf/libbpf-rs and CO-RE.

## Capture path
- No symbolization on sampling threads.
- No database writes on sampling threads.
- Avoid per-sample heap allocations where practical.
- All queues are bounded.
- PERF_RECORD_LOST must be measured and surfaced.

## Rust safety
- Keep unsafe code inside low-level crates.
- Every unsafe block requires a SAFETY comment explaining invariants.
- Prefer OwnedFd and RAII for kernel resources.
- No unwrap()/expect() in capture-path production code.

## Compatibility
- No x86-only assumptions.
- Every address is u64.
- Architecture-specific behavior must be isolated.
- AArch64 is a release-gating target.

## Quality gates
Run before declaring a task complete:
  cargo fmt --check
  cargo clippy --workspace --all-targets -- -D warnings
  cargo test --workspace

Add regression tests for every parser bug.

## Review output
At task completion report:
1. files changed;
2. design decisions;
3. tests executed;
4. unsafe code introduced;
5. unresolved risks.
