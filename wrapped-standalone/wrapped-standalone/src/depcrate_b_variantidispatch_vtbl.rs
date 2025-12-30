// Generated macro for IDispatch_Vtbl (struct)
macro_rules! Depcrate_b_variantIDispatch_Vtbl {
() => {
// Module: crate::b_variant
// Provides: {"IDispatch_Vtbl"}
// Dependencies: {}
# [repr (C)] pub struct IDispatch_Vtbl { pub base__ : IUnknown_Vtbl , pub GetTypeInfoCount : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut u32) -> HRESULT , GetTypeInfo : usize , pub GetIDsOfNames : unsafe extern "system" fn (* mut core :: ffi :: c_void , * const GUID , * const PCWSTR , u32 , u32 , * mut i32 ,) -> HRESULT , Invoke : usize , }
};
}
