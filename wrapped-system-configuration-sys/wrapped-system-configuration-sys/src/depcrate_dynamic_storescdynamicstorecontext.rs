// Generated macro for SCDynamicStoreContext (struct)
macro_rules! Depcrate_dynamic_storeSCDynamicStoreContext {
() => {
// Module: crate::dynamic_store
// Provides: {"SCDynamicStoreContext"}
// Dependencies: {}
# [repr (C)] pub struct SCDynamicStoreContext { pub version : CFIndex , pub info : * mut :: core :: ffi :: c_void , pub retain : Option < unsafe extern "C" fn (info : * const :: core :: ffi :: c_void) -> * const :: core :: ffi :: c_void , > , pub release : Option < unsafe extern "C" fn (info : * const :: core :: ffi :: c_void) > , pub copyDescription : Option < unsafe extern "C" fn (info : * const :: core :: ffi :: c_void) -> CFStringRef > , }
};
}
