# Linux Performance Profiler

Initial Rust workspace for a Linux-only performance profiler targeting x86_64
and aarch64. **Profiling is not implemented.** The scaffold itself has no native
or external Rust dependencies and can be checked on non-Linux development hosts.

## Build and test

Install a Rust toolchain with `rustfmt` and `clippy`, then run from this directory:

```sh
cargo build --workspace
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Placeholder commands (each prints that it is not implemented and exits successfully):

```sh
cargo run -p lpp-cli -- record
cargo run -p lpp-cli -- report
cargo run -p lpp-cli -- serve
cargo run -p lpp-cli -- --help
```

No capture, report generation, or network listener is started. Missing/unknown
commands and extra arguments exit with status 2.

## Workspace and dependency direction

| Crate | Responsibility |
| --- | --- |
| `lpp-cli` | Command-line entry point |
| `lpp-core` | Shared domain types only |
| `lpp-perf` | Future exclusive Linux perf UAPI integration |
| `lpp-unwind` | Future stack unwinding |
| `lpp-symbolize` | Future symbol resolution |
| `lpp-bpf` | Future exclusive libbpf-rs / CO-RE integration |
| `lpp-store` | Future profile persistence |
| `lpp-analysis` | Future profile analysis |
| `lpp-server` | Future serving API |

Every crate other than `lpp-core` currently depends only on `lpp-core`, which has
no dependencies. No other wiring is implemented. Platform integration must stay
behind its owning crate; shared types must not depend on platform or I/O code.
See [ADR 0001](docs/adr/0001-crate-boundaries.md),
[ADR 0002](docs/adr/0002-kernel-integration-boundaries.md), and
[the AI contract](docs/ai_contract.md).

`bpf/`, `native/`, and `ui/` reserve future source locations; `benches/` contains
only a standalone workload. Existing root `src/` files are preserved local work,
not members or build targets of this virtual workspace.

## CPU hotspot benchmark

Compile with GCC or Clang (a C11 compiler is required):

```sh
mkdir -p target/benches
cc -std=c11 -O2 -g -fno-omit-frame-pointer -Wall -Wextra -Werror benches/cpu_hotspot.c -o target/benches/cpu_hotspot
./target/benches/cpu_hotspot
```

Three `noinline` functions perform the same dependent integer operations with
7:2:1 iteration counts, targeting approximately 70%, 20%, and 10% of CPU work.
The printed checksum keeps the work observable. These are intended work shares,
not guaranteed measured percentages; compiler and CPU behavior can affect them.
The benchmark does not invoke or implement a profiler.
