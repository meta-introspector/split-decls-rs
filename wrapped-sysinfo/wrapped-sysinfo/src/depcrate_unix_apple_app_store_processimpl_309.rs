// Generated macro for impl_309 (impl)
macro_rules! Depcrate_unix_apple_app_store_processimpl_309 {
() => {
// Module: crate::unix::apple::app_store::process
// Provides: {"impl_309"}
// Dependencies: {}
impl ProcessInner { pub (crate) fn kill_with (& self , _signal : Signal) -> Option < bool > { None } pub (crate) fn name (& self) -> & OsStr { OsStr :: new ("") } pub (crate) fn cmd (& self) -> & [OsString] { & [] } pub (crate) fn exe (& self) -> Option < & Path > { None } pub (crate) fn pid (& self) -> Pid { Pid (0) } pub (crate) fn environ (& self) -> & [OsString] { & [] } pub (crate) fn cwd (& self) -> Option < & Path > { None } pub (crate) fn root (& self) -> Option < & Path > { None } pub (crate) fn memory (& self) -> u64 { 0 } pub (crate) fn virtual_memory (& self) -> u64 { 0 } pub (crate) fn parent (& self) -> Option < Pid > { None } pub (crate) fn status (& self) -> ProcessStatus { ProcessStatus :: Unknown (0) } pub (crate) fn start_time (& self) -> u64 { 0 } pub (crate) fn run_time (& self) -> u64 { 0 } pub (crate) fn cpu_usage (& self) -> f32 { 0.0 } pub (crate) fn accumulated_cpu_time (& self) -> u64 { 0 } pub (crate) fn disk_usage (& self) -> DiskUsage { DiskUsage :: default () } pub (crate) fn user_id (& self) -> Option < & Uid > { None } pub (crate) fn effective_user_id (& self) -> Option < & Uid > { None } pub (crate) fn group_id (& self) -> Option < Gid > { None } pub (crate) fn effective_group_id (& self) -> Option < Gid > { None } pub (crate) fn wait (& self) -> Option < ExitStatus > { None } pub (crate) fn session_id (& self) -> Option < Pid > { None } pub (crate) fn switch_updated (& mut self) -> bool { false } pub (crate) fn set_nonexistent (& mut self) { } pub (crate) fn exists (& self) -> bool { false } }
};
}
