// Generated macro for doctest (module)
macro_rules! Depcrate_common_systemdoctest {
() => {
// Module: crate::common::system
// Provides: {"doctest"}
// Dependencies: {}
# [cfg (doctest)] mod doctest { # [doc = " Check that `Process` doesn't implement `Clone`."] # [doc = ""] # [doc = " First we check that the \"basic\" code works:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use sysinfo::{Process, System};"] # [doc = ""] # [doc = " let mut s = System::new_all();"] # [doc = " let p: &Process = s.processes().values().next().unwrap();"] # [doc = " ```"] # [doc = ""] # [doc = " And now we check if it fails when we try to clone it:"] # [doc = ""] # [doc = " ```compile_fail"] # [doc = " use sysinfo::{Process, System};"] # [doc = ""] # [doc = " let mut s = System::new_all();"] # [doc = " let p: &Process = s.processes().values().next().unwrap();"] # [doc = " let p = (*p).clone();"] # [doc = " ```"] mod process_clone { } # [doc = " Check that `System` doesn't implement `Clone`."] # [doc = ""] # [doc = " First we check that the \"basic\" code works:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use sysinfo::{Process, System};"] # [doc = ""] # [doc = " let s = System::new();"] # [doc = " ```"] # [doc = ""] # [doc = " And now we check if it fails when we try to clone it:"] # [doc = ""] # [doc = " ```compile_fail"] # [doc = " use sysinfo::{Process, System};"] # [doc = ""] # [doc = " let s = System::new();"] # [doc = " let s = s.clone();"] # [doc = " ```"] mod system_clone { } }
};
}
