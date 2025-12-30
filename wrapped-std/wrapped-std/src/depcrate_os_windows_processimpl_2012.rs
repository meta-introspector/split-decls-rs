// Generated macro for impl_2012 (impl)
macro_rules! Depcrate_os_windows_processimpl_2012 {
() => {
// Module: crate::os::windows::process
// Provides: {"impl_2012"}
// Dependencies: {}
# [unstable (feature = "windows_process_exit_code_from" , issue = "111688")] impl ExitCodeExt for process :: ExitCode { fn from_raw (raw : u32) -> Self { process :: ExitCode :: from_inner (From :: from (raw)) } }
};
}
