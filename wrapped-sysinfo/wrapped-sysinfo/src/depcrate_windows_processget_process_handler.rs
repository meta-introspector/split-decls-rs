// Generated macro for get_process_handler (function)
macro_rules! Depcrate_windows_processget_process_handler {
() => {
// Module: crate::windows::process
// Provides: {"get_process_handler"}
// Dependencies: {}
fn get_process_handler (pid : Pid) -> Option < HandleWrapper > { if pid . 0 == 0 { return None ; } let options = PROCESS_QUERY_INFORMATION | PROCESS_VM_READ ; HandleWrapper :: new (unsafe { OpenProcess (options , false , pid . 0 as u32) . unwrap_or_default () }) . or_else (| | { sysinfo_debug ! ("OpenProcess failed, error: {:?}" , io :: Error :: last_os_error ()) ; HandleWrapper :: new (unsafe { OpenProcess (PROCESS_QUERY_LIMITED_INFORMATION , false , pid . 0 as u32) . unwrap_or_default () }) }) . or_else (| | { sysinfo_debug ! ("OpenProcess limited failed, error: {:?}" , io :: Error :: last_os_error ()) ; None }) }
};
}
