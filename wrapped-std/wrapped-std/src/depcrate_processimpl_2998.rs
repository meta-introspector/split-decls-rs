// Generated macro for impl_2998 (impl)
macro_rules! Depcrate_processimpl_2998 {
() => {
// Module: crate::process
// Provides: {"impl_2998"}
// Dependencies: {}
# [stable (feature = "stdio_from" , since = "1.20.0")] impl From < ChildStderr > for Stdio { # [doc = " Converts a [`ChildStderr`] into a [`Stdio`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " use std::process::{Command, Stdio};"] # [doc = ""] # [doc = " let reverse = Command::new(\"rev\")"] # [doc = "     .arg(\"non_existing_file.txt\")"] # [doc = "     .stderr(Stdio::piped())"] # [doc = "     .spawn()"] # [doc = "     .expect(\"failed reverse command\");"] # [doc = ""] # [doc = " let cat = Command::new(\"cat\")"] # [doc = "     .arg(\"-\")"] # [doc = "     .stdin(reverse.stderr.unwrap()) // Converted into a Stdio here"] # [doc = "     .output()"] # [doc = "     .expect(\"failed echo command\");"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     String::from_utf8_lossy(&cat.stdout),"] # [doc = "     \"rev: cannot open non_existing_file.txt: No such file or directory\\n\""] # [doc = " );"] # [doc = " ```"] fn from (child : ChildStderr) -> Stdio { Stdio :: from_inner (child . into_inner () . into ()) } }
};
}
