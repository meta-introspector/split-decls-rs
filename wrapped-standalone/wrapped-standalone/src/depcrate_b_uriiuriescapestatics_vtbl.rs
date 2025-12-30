// Generated macro for IUriEscapeStatics_Vtbl (struct)
macro_rules! Depcrate_b_uriIUriEscapeStatics_Vtbl {
() => {
// Module: crate::b_uri
// Provides: {"IUriEscapeStatics_Vtbl"}
// Dependencies: {}
# [repr (C)] # [doc (hidden)] pub struct IUriEscapeStatics_Vtbl { pub base__ : windows_core :: IInspectable_Vtbl , pub UnescapeComponent : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut core :: ffi :: c_void , * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT , pub EscapeComponent : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut core :: ffi :: c_void , * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT , }
};
}
