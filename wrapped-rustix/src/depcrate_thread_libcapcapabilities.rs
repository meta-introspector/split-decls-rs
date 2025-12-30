// Generated macro for capabilities (function)
macro_rules! Depcrate_thread_libcapcapabilities {
() => {
// Module: crate::thread::libcap
// Provides: {"capabilities"}
// Dependencies: {}
# [doc = " `capget(_LINUX_CAPABILITY_VERSION_3, pid)`"] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/capget.2.html"] # [inline] # [doc (alias = "capget")] pub fn capabilities (pid : Option < Pid >) -> io :: Result < CapabilitySets > { capget (pid) }
};
}
