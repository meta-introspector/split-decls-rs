// Generated macro for sysinfo_process_by_pid (function)
macro_rules! Depcrate_c_interfacesysinfo_process_by_pid {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_process_by_pid"}
// Dependencies: {}
# [doc = " Equivalent of [`System::process()`][crate::System#method.process]."] # [doc = ""] # [doc = " # ⚠\u{fe0f} WARNING ⚠\u{fe0f}"] # [doc = ""] # [doc = " While having this method returned process, you should *never* call any"] # [doc = " refresh method!"] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_process_by_pid (system : CSystem , pid : PID) -> CProcess { assert ! (! system . is_null ()) ; unsafe { let system : Box < System > = Box :: from_raw (system as * mut System) ; let ret = if let Some (process) = system . process (Pid (pid as _)) { process as * const Process as CProcess } else { std :: ptr :: null () } ; let _ = Box :: into_raw (system) ; ret } }
};
}
