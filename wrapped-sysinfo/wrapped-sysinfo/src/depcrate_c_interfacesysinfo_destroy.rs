// Generated macro for sysinfo_destroy (function)
macro_rules! Depcrate_c_interfacesysinfo_destroy {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_destroy"}
// Dependencies: {}
# [doc = " Equivalent of `System::drop()`. Important in C to cleanup memory."] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_destroy (system : CSystem) { assert ! (! system . is_null ()) ; unsafe { drop (Box :: from_raw (system as * mut System)) ; } }
};
}
