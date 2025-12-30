// Generated macro for sysinfo_refresh_memory (function)
macro_rules! Depcrate_c_interfacesysinfo_refresh_memory {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_refresh_memory"}
// Dependencies: {}
# [doc = " Equivalent of [`System::refresh_memory()`][crate::System#method.refresh_memory]."] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_refresh_memory (system : CSystem) { assert ! (! system . is_null ()) ; unsafe { let mut system : Box < System > = Box :: from_raw (system as * mut System) ; { let system : & mut System = system . borrow_mut () ; system . refresh_memory () ; } let _ = Box :: into_raw (system) ; } }
};
}
