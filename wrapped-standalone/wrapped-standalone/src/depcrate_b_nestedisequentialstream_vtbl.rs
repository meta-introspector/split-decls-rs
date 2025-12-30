// Generated macro for ISequentialStream_Vtbl (struct)
macro_rules! Depcrate_b_nestedISequentialStream_Vtbl {
() => {
// Module: crate::b_nested
// Provides: {"ISequentialStream_Vtbl"}
// Dependencies: {}
# [repr (C)] pub struct ISequentialStream_Vtbl { pub base__ : IUnknown_Vtbl , pub Read : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut core :: ffi :: c_void , u32 , * mut u32 ,) -> HRESULT , pub Write : unsafe extern "system" fn (* mut core :: ffi :: c_void , * const core :: ffi :: c_void , u32 , * mut u32 ,) -> HRESULT , }
};
}
