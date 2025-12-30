// Generated macro for impl_2006 (impl)
macro_rules! Depcrate_os_windows_processimpl_2006 {
() => {
// Module: crate::os::windows::process
// Provides: {"impl_2006"}
// Dependencies: {}
# [stable (feature = "exit_status_from" , since = "1.12.0")] impl ExitStatusExt for process :: ExitStatus { fn from_raw (raw : u32) -> Self { process :: ExitStatus :: from_inner (From :: from (raw)) } }
};
}
