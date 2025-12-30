// Generated macro for current_os_id (function)
macro_rules! Depcrate_thread_currentcurrent_os_id {
() => {
// Module: crate::thread::current
// Provides: {"current_os_id"}
// Dependencies: {}
# [doc = " Gets the OS thread ID of the thread that invokes it, if available. If not, return the Rust"] # [doc = " thread ID."] # [doc = ""] # [doc = " We use a `u64` to all possible platform IDs without excess `cfg`; most use `int`, some use a"] # [doc = " pointer, and Apple uses `uint64_t`. This is a \"best effort\" approach for diagnostics and is"] # [doc = " allowed to fall back to a non-OS ID (such as the Rust thread ID) or a non-unique ID (such as a"] # [doc = " PID) if the thread ID cannot be retrieved."] pub (crate) fn current_os_id () -> u64 { imp :: current_os_id () . unwrap_or_else (| | current_id () . as_u64 () . get ()) }
};
}
