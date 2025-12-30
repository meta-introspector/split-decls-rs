// Generated macro for impl_2997 (impl)
macro_rules! Depcrate_processimpl_2997 {
() => {
// Module: crate::process
// Provides: {"impl_2997"}
// Dependencies: {}
# [stable (feature = "stdio_from" , since = "1.20.0")] impl From < ChildStdout > for Stdio { # [doc = " Converts a [`ChildStdout`] into a [`Stdio`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " `ChildStdout` will be converted to `Stdio` using `Stdio::from` under the hood."] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " use std::process::{Command, Stdio};"] # [doc = ""] # [doc = " let hello = Command::new(\"echo\")"] # [doc = "     .arg(\"Hello, world!\")"] # [doc = "     .stdout(Stdio::piped())"] # [doc = "     .spawn()"] # [doc = "     .expect(\"failed echo command\");"] # [doc = ""] # [doc = " let reverse = Command::new(\"rev\")"] # [doc = "     .stdin(hello.stdout.unwrap())  // Converted into a Stdio here"] # [doc = "     .output()"] # [doc = "     .expect(\"failed reverse command\");"] # [doc = ""] # [doc = " assert_eq!(reverse.stdout, b\"!dlrow ,olleH\\n\");"] # [doc = " ```"] fn from (child : ChildStdout) -> Stdio { Stdio :: from_inner (child . into_inner () . into ()) } }
};
}
