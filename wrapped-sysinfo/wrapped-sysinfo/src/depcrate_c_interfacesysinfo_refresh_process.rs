// Generated macro for sysinfo_refresh_process (function)
macro_rules! Depcrate_c_interfacesysinfo_refresh_process {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_refresh_process"}
// Dependencies: {}
# [doc = " Equivalent of [`System::refresh_processes(ProcessesToUpdate::Some(pid))`]."] # [doc = ""] # [doc = " [`System::refresh_processes(ProcessesToUpdate::Some(pid))`]: crate::System#method.refresh_processes"] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_refresh_process (system : CSystem , pid : PID) { assert ! (! system . is_null ()) ; unsafe { let mut system : Box < System > = Box :: from_raw (system as * mut System) ; { let system : & mut System = system . borrow_mut () ; system . refresh_processes (ProcessesToUpdate :: Some (& [Pid :: from_u32 (pid as _)]) , true) ; } let _ = Box :: into_raw (system) ; } }
};
}
