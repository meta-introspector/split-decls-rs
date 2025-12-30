// Generated macro for id (function)
macro_rules! Depcrate_processid {
() => {
// Module: crate::process
// Provides: {"id"}
// Dependencies: {}
# [doc = " Returns the OS-assigned process identifier associated with this process."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::process;"] # [doc = ""] # [doc = " println!(\"My pid is {}\", process::id());"] # [doc = " ```"] # [must_use] # [stable (feature = "getpid" , since = "1.26.0")] pub fn id () -> u32 { crate :: sys :: os :: getpid () }
};
}
