// Generated macro for sysinfo_free_memory (function)
macro_rules! Depcrate_c_interfacesysinfo_free_memory {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_free_memory"}
// Dependencies: {}
# [doc = " Equivalent of [`System::free_memory()`][crate::System#method.free_memory]."] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_free_memory (system : CSystem) -> size_t { assert ! (! system . is_null ()) ; unsafe { let system : Box < System > = Box :: from_raw (system as * mut System) ; let ret = system . free_memory () as size_t ; let _ = Box :: into_raw (system) ; ret } }
};
}
