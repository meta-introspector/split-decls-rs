// Generated macro for ChildExt (trait)
macro_rules! Depcrate_os_unix_processChildExt {
() => {
// Module: crate::os::unix::process
// Provides: {"ChildExt"}
// Dependencies: {}
# [unstable (feature = "unix_send_signal" , issue = "141975")] pub trait ChildExt : Sealed { # [doc = " Sends a signal to a child process."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function will return an error if the signal is invalid. The integer values associated"] # [doc = " with signals are implementation-specific, so it's encouraged to use a crate that provides"] # [doc = " posix bindings."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " #![feature(unix_send_signal)]"] # [doc = ""] # [doc = " use std::{io, os::unix::process::ChildExt, process::{Command, Stdio}};"] # [doc = ""] # [doc = " use libc::SIGTERM;"] # [doc = ""] # [doc = " fn main() -> io::Result<()> {"] # [doc = "     let child = Command::new(\"cat\").stdin(Stdio::piped()).spawn()?;"] # [doc = "     child.send_signal(SIGTERM)?;"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] fn send_signal (& self , signal : i32) -> io :: Result < () > ; }
};
}
