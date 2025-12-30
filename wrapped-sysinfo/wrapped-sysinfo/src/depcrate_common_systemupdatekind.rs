// Generated macro for UpdateKind (enum)
macro_rules! Depcrate_common_systemUpdateKind {
() => {
// Module: crate::common::system
// Provides: {"UpdateKind"}
// Dependencies: {}
# [doc = " This enum allows you to specify when you want the related information to be updated."] # [doc = ""] # [doc = " For example if you only want the [`Process::exe()`] information to be refreshed only if it's not"] # [doc = " already set:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use sysinfo::{ProcessesToUpdate, ProcessRefreshKind, System, UpdateKind};"] # [doc = ""] # [doc = " let mut system = System::new();"] # [doc = " system.refresh_processes_specifics("] # [doc = "     ProcessesToUpdate::All,"] # [doc = "     true,"] # [doc = "     ProcessRefreshKind::nothing().with_exe(UpdateKind::OnlyIfNotSet),"] # [doc = " );"] # [doc = " ```"] # [derive (Clone , Copy , Debug , Default , PartialEq , Eq)] pub enum UpdateKind { # [doc = " Never update the related information."] # [default] Never , # [doc = " Always update the related information."] Always , # [doc = " Only update the related information if it was not already set at least once."] OnlyIfNotSet , }
};
}
