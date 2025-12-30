// Generated macro for ExitCodeExt (trait)
macro_rules! Depcrate_os_windows_processExitCodeExt {
() => {
// Module: crate::os::windows::process
// Provides: {"ExitCodeExt"}
// Dependencies: {}
# [doc = " Windows-specific extensions to [`process::ExitCode`]."] # [doc = ""] # [doc = " This trait is sealed: it cannot be implemented outside the standard library."] # [doc = " This is so that future additional methods are not breaking changes."] # [unstable (feature = "windows_process_exit_code_from" , issue = "111688")] pub trait ExitCodeExt : Sealed { # [doc = " Creates a new `ExitCode` from the raw underlying `u32` return value of"] # [doc = " a process."] # [doc = ""] # [doc = " The exit code should not be 259, as this conflicts with the `STILL_ACTIVE`"] # [doc = " macro returned from the `GetExitCodeProcess` function to signal that the"] # [doc = " process has yet to run to completion."] # [unstable (feature = "windows_process_exit_code_from" , issue = "111688")] fn from_raw (raw : u32) -> Self ; }
};
}
