// Generated macro for sysinfo_total_memory (function)
macro_rules! Depcrate_c_interfacesysinfo_total_memory {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_total_memory"}
// Dependencies: {}
# [doc = " Equivalent of [`System::total_memory()`][crate::System#method.total_memory]."] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_total_memory (system : CSystem) -> size_t { assert ! (! system . is_null ()) ; unsafe { let system : Box < System > = Box :: from_raw (system as * mut System) ; let ret = system . total_memory () as size_t ; let _ = Box :: into_raw (system) ; ret } }
};
}
