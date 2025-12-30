// Generated macro for impl_2996 (impl)
macro_rules! Depcrate_processimpl_2996 {
() => {
// Module: crate::process
// Provides: {"impl_2996"}
// Dependencies: {}
# [stable (feature = "stdio_from" , since = "1.20.0")] impl From < ChildStdin > for Stdio { # [doc = " Converts a [`ChildStdin`] into a [`Stdio`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " `ChildStdin` will be converted to `Stdio` using `Stdio::from` under the hood."] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " use std::process::{Command, Stdio};"] # [doc = ""] # [doc = " let reverse = Command::new(\"rev\")"] # [doc = "     .stdin(Stdio::piped())"] # [doc = "     .spawn()"] # [doc = "     .expect(\"failed reverse command\");"] # [doc = ""] # [doc = " let _echo = Command::new(\"echo\")"] # [doc = "     .arg(\"Hello, world!\")"] # [doc = "     .stdout(reverse.stdin.unwrap()) // Converted into a Stdio here"] # [doc = "     .output()"] # [doc = "     .expect(\"failed echo command\");"] # [doc = ""] # [doc = " // \"!dlrow ,olleH\" echoed to console"] # [doc = " ```"] fn from (child : ChildStdin) -> Stdio { Stdio :: from_inner (child . into_inner () . into ()) } }
};
}
