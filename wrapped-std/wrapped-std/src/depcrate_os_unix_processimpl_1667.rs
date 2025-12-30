// Generated macro for impl_1667 (impl)
macro_rules! Depcrate_os_unix_processimpl_1667 {
() => {
// Module: crate::os::unix::process
// Provides: {"impl_1667"}
// Dependencies: {}
# [unstable (feature = "exit_status_error" , issue = "84908")] impl ExitStatusExt for process :: ExitStatusError { fn from_raw (raw : i32) -> Self { process :: ExitStatus :: from_raw (raw) . exit_ok () . expect_err ("<ExitStatusError as ExitStatusExt>::from_raw(0) but zero is not an error") } fn signal (& self) -> Option < i32 > { self . into_status () . signal () } fn core_dumped (& self) -> bool { self . into_status () . core_dumped () } fn stopped_signal (& self) -> Option < i32 > { self . into_status () . stopped_signal () } fn continued (& self) -> bool { self . into_status () . continued () } fn into_raw (self) -> i32 { self . into_status () . into_raw () } }
};
}
