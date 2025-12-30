// Generated macro for ProcessesToUpdate (enum)
macro_rules! Depcrate_common_systemProcessesToUpdate {
() => {
// Module: crate::common::system
// Provides: {"ProcessesToUpdate"}
// Dependencies: {}
# [doc = " This enum allows you to specify if you want all processes to be updated or just"] # [doc = " some of them."] # [doc = ""] # [doc = " Example:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use sysinfo::{ProcessesToUpdate, System, get_current_pid};"] # [doc = ""] # [doc = " let mut system = System::new();"] # [doc = " // To refresh all processes:"] # [doc = " system.refresh_processes(ProcessesToUpdate::All, true);"] # [doc = ""] # [doc = " // To refresh only the current one:"] # [doc = " system.refresh_processes("] # [doc = "     ProcessesToUpdate::Some(&[get_current_pid().unwrap()]),"] # [doc = "     true,"] # [doc = " );"] # [doc = " ```"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum ProcessesToUpdate < 'a > { # [doc = " To refresh all processes."] All , # [doc = " To refresh only the processes with the listed [`Pid`]."] # [doc = ""] # [doc = " [`Pid`]: crate::Pid"] Some (& 'a [Pid]) , }
};
}
