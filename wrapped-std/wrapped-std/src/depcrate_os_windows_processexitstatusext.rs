// Generated macro for ExitStatusExt (trait)
macro_rules! Depcrate_os_windows_processExitStatusExt {
() => {
// Module: crate::os::windows::process
// Provides: {"ExitStatusExt"}
// Dependencies: {}
# [doc = " Windows-specific extensions to [`process::ExitStatus`]."] # [doc = ""] # [doc = " This trait is sealed: it cannot be implemented outside the standard library."] # [doc = " This is so that future additional methods are not breaking changes."] # [stable (feature = "exit_status_from" , since = "1.12.0")] pub trait ExitStatusExt : Sealed { # [doc = " Creates a new `ExitStatus` from the raw underlying `u32` return value of"] # [doc = " a process."] # [stable (feature = "exit_status_from" , since = "1.12.0")] fn from_raw (raw : u32) -> Self ; }
};
}
