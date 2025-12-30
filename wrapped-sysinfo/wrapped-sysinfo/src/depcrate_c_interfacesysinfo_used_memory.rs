// Generated macro for sysinfo_used_memory (function)
macro_rules! Depcrate_c_interfacesysinfo_used_memory {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_used_memory"}
// Dependencies: {}
# [doc = " Equivalent of [`System::used_memory()`][crate::System#method.used_memory]."] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_used_memory (system : CSystem) -> size_t { assert ! (! system . is_null ()) ; let system : Box < System > = unsafe { Box :: from_raw (system as * mut System) } ; let ret = system . used_memory () as size_t ; let _ = Box :: into_raw (system) ; ret }
};
}
