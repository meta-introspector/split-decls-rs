// Generated macro for SCPreferencesContext (struct)
macro_rules! Depcrate_preferencesSCPreferencesContext {
() => {
// Module: crate::preferences
// Provides: {"SCPreferencesContext"}
// Dependencies: {}
# [repr (C)] pub struct SCPreferencesContext { pub version : CFIndex , pub info : * mut :: core :: ffi :: c_void , pub retain : Option < unsafe extern "C" fn (info : * const :: core :: ffi :: c_void) -> * const :: core :: ffi :: c_void , > , pub release : Option < unsafe extern "C" fn (info : * const :: core :: ffi :: c_void) > , pub copyDescription : Option < unsafe extern "C" fn (info : * const :: core :: ffi :: c_void) -> CFStringRef > , }
};
}
