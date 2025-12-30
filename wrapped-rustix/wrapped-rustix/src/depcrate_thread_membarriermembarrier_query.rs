// Generated macro for membarrier_query (function)
macro_rules! Depcrate_thread_membarriermembarrier_query {
() => {
// Module: crate::thread::membarrier
// Provides: {"membarrier_query"}
// Dependencies: {}
# [doc = " `membarrier(MEMBARRIER_CMD_QUERY, 0, 0)`—Query the supported `membarrier`"] # [doc = " commands."] # [doc = ""] # [doc = " This function doesn't return a `Result` because it always succeeds; if the"] # [doc = " underlying OS doesn't support the `membarrier` syscall, it returns an empty"] # [doc = " `MembarrierQuery` value."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/membarrier.2.html"] # [inline] # [doc (alias = "MEMBARRIER_CMD_QUERY")] pub fn membarrier_query () -> MembarrierQuery { backend :: thread :: syscalls :: membarrier_query () }
};
}
