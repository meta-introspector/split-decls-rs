// Generated macro for Process (struct)
macro_rules! Depcrate_common_systemProcess {
() => {
// Module: crate::common::system
// Provides: {"Process"}
// Dependencies: {}
# [doc = " Struct containing information of a process."] # [doc = ""] # [doc = " ## iOS"] # [doc = ""] # [doc = " This information cannot be retrieved on iOS due to sandboxing."] # [doc = ""] # [doc = " ## Apple app store"] # [doc = ""] # [doc = " If you are building a macOS Apple app store, it won't be able"] # [doc = " to retrieve this information."] # [doc = ""] # [doc = " ```no_run"] # [doc = " use sysinfo::{Pid, System};"] # [doc = ""] # [doc = " let s = System::new_all();"] # [doc = " if let Some(process) = s.process(Pid::from(1337)) {"] # [doc = "     println!(\"{:?}\", process.name());"] # [doc = " }"] # [doc = " ```"] pub struct Process { pub (crate) inner : ProcessInner , }
};
}
