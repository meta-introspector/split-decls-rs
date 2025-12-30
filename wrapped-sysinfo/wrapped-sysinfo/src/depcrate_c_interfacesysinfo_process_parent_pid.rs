// Generated macro for sysinfo_process_parent_pid (function)
macro_rules! Depcrate_c_interfacesysinfo_process_parent_pid {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_process_parent_pid"}
// Dependencies: {}
# [doc = " Equivalent of [`Process::parent()`][crate::Process#method.parent]."] # [doc = ""] # [doc = " In case there is no known parent, it returns `0`."] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_process_parent_pid (process : CProcess) -> PID { assert ! (! process . is_null ()) ; let process = process as * const Process ; unsafe { (* process) . parent () . unwrap_or (Pid (0)) . 0 as _ } }
};
}
