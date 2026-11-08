# ADR 0002: Exclusive ownership of perf UAPI and libbpf

Status: Accepted

## Context

Linux perf and eBPF integration involve platform-specific APIs and resource
lifetimes. These must not leak across the workspace or into shared domain types.

## Decision

Only `lpp-perf` may directly depend on Linux perf UAPI. This includes binding
crates, generated or handwritten perf ABI structures/constants, and direct
`perf_event_open` calls. Future capture must integrate directly, not invoke the
`perf` executable. Convert kernel records to domain types at this boundary.

Only `lpp-bpf` may directly depend on `libbpf-rs`. Future production eBPF uses
libbpf/libbpf-rs and CO-RE. Keep associated skeleton/build integration inside
this ownership boundary; `bpf/` is reserved for future C eBPF sources.

Other crates may consume safe APIs of these owners but must not bypass them
through re-exports, duplicated bindings, or native shims. `native/` is reserved
for narrow FFI shims, not an alternative owner of perf or libbpf integration.
Prefer OwnedFd and RAII when kernel resources are introduced.

There are no perf UAPI or libbpf dependencies, bindings, build scripts, syscalls,
or native integrations in this initial scaffold. These are architectural rules
for subsequent implementation, not a claim of an automated dependency audit.

## Consequences

The scaffold can build and test on a development host without Linux headers or
libbpf. This does not promise non-Linux runtime support: production targets stay
Linux x86_64 and aarch64, with aarch64 a release gate. Future platform-dependent
code must be gated and tested accordingly.

The standalone C CPU workload under `benches/` is a benchmark fixture, not
production profiler code or an FFI integration.
