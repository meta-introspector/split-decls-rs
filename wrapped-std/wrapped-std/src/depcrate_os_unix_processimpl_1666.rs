// Generated macro for impl_1666 (impl)
macro_rules! Depcrate_os_unix_processimpl_1666 {
() => {
// Module: crate::os::unix::process
// Provides: {"impl_1666"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl ExitStatusExt for process :: ExitStatus { fn from_raw (raw : i32) -> Self { process :: ExitStatus :: from_inner (From :: from (raw)) } fn signal (& self) -> Option < i32 > { self . as_inner () . signal () } fn core_dumped (& self) -> bool { self . as_inner () . core_dumped () } fn stopped_signal (& self) -> Option < i32 > { self . as_inner () . stopped_signal () } fn continued (& self) -> bool { self . as_inner () . continued () } fn into_raw (self) -> i32 { self . as_inner () . into_raw () . into () } }
};
}
