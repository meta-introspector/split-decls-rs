// Generated macro for IUriRuntimeClassFactory_Vtbl (struct)
macro_rules! Depcrate_b_uriIUriRuntimeClassFactory_Vtbl {
() => {
// Module: crate::b_uri
// Provides: {"IUriRuntimeClassFactory_Vtbl"}
// Dependencies: {}
# [repr (C)] # [doc (hidden)] pub struct IUriRuntimeClassFactory_Vtbl { pub base__ : windows_core :: IInspectable_Vtbl , pub CreateUri : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut core :: ffi :: c_void , * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT , pub CreateWithRelativeUri : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut core :: ffi :: c_void , * mut core :: ffi :: c_void , * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT , }
};
}
