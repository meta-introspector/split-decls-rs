// Generated macro for set_capabilities (function)
macro_rules! Depcrate_thread_libcapset_capabilities {
() => {
// Module: crate::thread::libcap
// Provides: {"set_capabilities"}
// Dependencies: {}
# [doc = " `capset(_LINUX_CAPABILITY_VERSION_3, pid, effective, permitted,"] # [doc = " inheritable)`"] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/capget.2.html"] # [inline] # [doc (alias = "capset")] pub fn set_capabilities (pid : Option < Pid > , sets : CapabilitySets) -> io :: Result < () > { capset (pid , sets) }
};
}
