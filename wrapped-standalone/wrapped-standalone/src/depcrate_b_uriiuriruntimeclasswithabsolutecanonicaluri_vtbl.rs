// Generated macro for IUriRuntimeClassWithAbsoluteCanonicalUri_Vtbl (struct)
macro_rules! Depcrate_b_uriIUriRuntimeClassWithAbsoluteCanonicalUri_Vtbl {
() => {
// Module: crate::b_uri
// Provides: {"IUriRuntimeClassWithAbsoluteCanonicalUri_Vtbl"}
// Dependencies: {}
# [repr (C)] # [doc (hidden)] pub struct IUriRuntimeClassWithAbsoluteCanonicalUri_Vtbl { pub base__ : windows_core :: IInspectable_Vtbl , pub AbsoluteCanonicalUri : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT , pub DisplayIri : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT , }
};
}
