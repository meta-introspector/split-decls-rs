// Generated macro for sysinfo_refresh_processes (function)
macro_rules! Depcrate_c_interfacesysinfo_refresh_processes {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_refresh_processes"}
// Dependencies: {}
# [doc = " Equivalent of [`System::refresh_processes(ProcessesToUpdate::All)`]."] # [doc = ""] # [doc = " [`System::refresh_processes(ProcessesToUpdate::All)`]: crate::System#method.refresh_processes"] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_refresh_processes (system : CSystem) { assert ! (! system . is_null ()) ; unsafe { let mut system : Box < System > = Box :: from_raw (system as * mut System) ; { let system : & mut System = system . borrow_mut () ; system . refresh_processes (ProcessesToUpdate :: All , true) ; } let _ = Box :: into_raw (system) ; } }
};
}
