// Generated macro for sysinfo_total_swap (function)
macro_rules! Depcrate_c_interfacesysinfo_total_swap {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_total_swap"}
// Dependencies: {}
# [doc = " Equivalent of [`System::total_swap()`][crate::System#method.total_swap]."] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_total_swap (system : CSystem) -> size_t { assert ! (! system . is_null ()) ; unsafe { let system : Box < System > = Box :: from_raw (system as * mut System) ; let ret = system . total_swap () as size_t ; let _ = Box :: into_raw (system) ; ret } }
};
}
