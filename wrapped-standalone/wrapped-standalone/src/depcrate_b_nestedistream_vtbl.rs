// Generated macro for IStream_Vtbl (struct)
macro_rules! Depcrate_b_nestedIStream_Vtbl {
() => {
// Module: crate::b_nested
// Provides: {"IStream_Vtbl"}
// Dependencies: {}
# [repr (C)] pub struct IStream_Vtbl { pub base__ : ISequentialStream_Vtbl , Seek : usize , pub SetSize : unsafe extern "system" fn (* mut core :: ffi :: c_void , u64) -> HRESULT , pub CopyTo : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut core :: ffi :: c_void , u64 , * mut u64 , * mut u64 ,) -> HRESULT , Commit : usize , pub Revert : unsafe extern "system" fn (* mut core :: ffi :: c_void) -> HRESULT , LockRegion : usize , pub UnlockRegion : unsafe extern "system" fn (* mut core :: ffi :: c_void , u64 , u64 , u32) -> HRESULT , Stat : usize , pub Clone : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut * mut core :: ffi :: c_void) -> HRESULT , }
};
}
