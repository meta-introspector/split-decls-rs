// Generated macro for sysinfo_process_cpu_usage (function)
macro_rules! Depcrate_c_interfacesysinfo_process_cpu_usage {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_process_cpu_usage"}
// Dependencies: {}
# [doc = " Equivalent of [`Process::cpu_usage()`][crate::Process#method.cpu_usage]."] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_process_cpu_usage (process : CProcess) -> c_float { assert ! (! process . is_null ()) ; let process = process as * const Process ; unsafe { (* process) . cpu_usage () } }
};
}
