// Generated macro for sysinfo_refresh_all (function)
macro_rules! Depcrate_c_interfacesysinfo_refresh_all {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_refresh_all"}
// Dependencies: {}
# [doc = " Equivalent of [`System::refresh_all()`][crate::System#method.refresh_all]."] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_refresh_all (system : CSystem) { assert ! (! system . is_null ()) ; unsafe { let mut system : Box < System > = Box :: from_raw (system as * mut System) ; { let system : & mut System = system . borrow_mut () ; system . refresh_all () ; } let _ = Box :: into_raw (system) ; } }
};
}
