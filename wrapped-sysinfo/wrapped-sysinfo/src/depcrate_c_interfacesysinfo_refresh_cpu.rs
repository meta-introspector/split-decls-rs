// Generated macro for sysinfo_refresh_cpu (function)
macro_rules! Depcrate_c_interfacesysinfo_refresh_cpu {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_refresh_cpu"}
// Dependencies: {}
# [doc = " Equivalent of [`System::refresh_cpu_usage()`][crate::System#method.refresh_cpu_usage]."] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_refresh_cpu (system : CSystem) { assert ! (! system . is_null ()) ; unsafe { let mut system : Box < System > = Box :: from_raw (system as * mut System) ; { let system : & mut System = system . borrow_mut () ; system . refresh_cpu_usage () ; } let _ = Box :: into_raw (system) ; } }
};
}
