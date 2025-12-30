// Generated macro for sysinfo_used_swap (function)
macro_rules! Depcrate_c_interfacesysinfo_used_swap {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_used_swap"}
// Dependencies: {}
# [doc = " Equivalent of [`System::used_swap()`][crate::System#method.used_swap]."] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_used_swap (system : CSystem) -> size_t { assert ! (! system . is_null ()) ; unsafe { let system : Box < System > = Box :: from_raw (system as * mut System) ; let ret = system . used_swap () as size_t ; let _ = Box :: into_raw (system) ; ret } }
};
}
