// Generated macro for impl_2990 (impl)
macro_rules! Depcrate_processimpl_2990 {
() => {
// Module: crate::process
// Provides: {"impl_2990"}
// Dependencies: {}
impl Output { # [doc = " Returns an error if a nonzero exit status was received."] # [doc = ""] # [doc = " If the [`Command`] exited successfully,"] # [doc = " `self` is returned."] # [doc = ""] # [doc = " This is equivalent to calling [`exit_ok`](ExitStatus::exit_ok)"] # [doc = " on [`Output.status`](Output::status)."] # [doc = ""] # [doc = " Note that this will throw away the [`Output::stderr`] field in the error case."] # [doc = " If the child process outputs useful informantion to stderr, you can:"] # [doc = " * Use `cmd.stderr(Stdio::inherit())` to forward the"] # [doc = "   stderr child process to the parent's stderr,"] # [doc = "   usually printing it to console where the user can see it."] # [doc = "   This is usually correct for command-line applications."] # [doc = " * Capture `stderr` using a custom error type."] # [doc = "   This is usually correct for libraries."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(exit_status_error)]"] # [doc = " # #[cfg(all(unix, not(target_os = \"android\")))] {"] # [doc = " use std::process::Command;"] # [doc = " assert!(Command::new(\"false\").output().unwrap().exit_ok().is_err());"] # [doc = " # }"] # [doc = " ```"] # [unstable (feature = "exit_status_error" , issue = "84908")] pub fn exit_ok (self) -> Result < Self , ExitStatusError > { self . status . exit_ok () ? ; Ok (self) } }
};
}
