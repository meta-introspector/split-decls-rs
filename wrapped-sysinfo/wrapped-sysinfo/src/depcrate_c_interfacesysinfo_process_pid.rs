// Generated macro for sysinfo_process_pid (function)
macro_rules! Depcrate_c_interfacesysinfo_process_pid {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_process_pid"}
// Dependencies: {}
# [doc = " Equivalent of [`Process::pid()`][crate::Process#method.pid]."] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_process_pid (process : CProcess) -> PID { assert ! (! process . is_null ()) ; let process = process as * const Process ; unsafe { (* process) . pid () . 0 as _ } }
};
}
