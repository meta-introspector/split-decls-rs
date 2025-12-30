// Generated macro for membarrier (function)
macro_rules! Depcrate_thread_membarriermembarrier {
() => {
// Module: crate::thread::membarrier
// Provides: {"membarrier"}
// Dependencies: {}
# [doc = " `membarrier(cmd, 0, 0)`—Perform a memory barrier."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/membarrier.2.html"] # [inline] pub fn membarrier (cmd : MembarrierCommand) -> io :: Result < () > { backend :: thread :: syscalls :: membarrier (cmd) }
};
}
