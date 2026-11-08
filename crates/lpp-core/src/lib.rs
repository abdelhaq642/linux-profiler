//! Shared domain types only; no platform, I/O, or orchestration dependencies.

/// A userspace or kernel address, independent of the host pointer width.
pub type Address = u64;
