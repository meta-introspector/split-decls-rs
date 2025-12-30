// Generated macro for sysinfo_free_swap (function)
macro_rules! Depcrate_c_interfacesysinfo_free_swap {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_free_swap"}
// Dependencies: {}
# [doc = " Equivalent of [`System::free_swap()`][crate::System#method.free_swap]."] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_free_swap (system : CSystem) -> size_t { assert ! (! system . is_null ()) ; unsafe { let system : Box < System > = Box :: from_raw (system as * mut System) ; let ret = system . free_swap () as size_t ; let _ = Box :: into_raw (system) ; ret } }
};
}
