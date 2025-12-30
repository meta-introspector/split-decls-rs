// Generated macro for get_parent (function)
macro_rules! Depcrate_unix_apple_macos_processget_parent {
() => {
// Module: crate::unix::apple::macos::process
// Provides: {"get_parent"}
// Dependencies: {}
fn get_parent (info : & libc :: proc_bsdinfo) -> Option < Pid > { match info . pbi_ppid as i32 { 0 => None , p => Some (Pid (p)) , } }
};
}
