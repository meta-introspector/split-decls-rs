// Generated macro for impl_1669 (impl)
macro_rules! Depcrate_os_unix_processimpl_1669 {
() => {
// Module: crate::os::unix::process
// Provides: {"impl_1669"}
// Dependencies: {}
# [unstable (feature = "unix_send_signal" , issue = "141975")] impl ChildExt for process :: Child { fn send_signal (& self , signal : i32) -> io :: Result < () > { self . handle . send_signal (signal) } }
};
}
