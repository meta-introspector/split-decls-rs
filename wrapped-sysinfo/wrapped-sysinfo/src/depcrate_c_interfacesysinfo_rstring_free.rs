// Generated macro for sysinfo_rstring_free (function)
macro_rules! Depcrate_c_interfacesysinfo_rstring_free {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_rstring_free"}
// Dependencies: {}
# [doc = " Frees a C string created with `CString::into_raw()`."] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_rstring_free (s : RString) { if ! s . is_null () { unsafe { let _ = CString :: from_raw (s as usize as * mut _) ; } } }
};
}
