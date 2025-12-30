// Generated macro for IPersistFile_Vtbl (struct)
macro_rules! Depcrate_b_vtbl_4IPersistFile_Vtbl {
() => {
// Module: crate::b_vtbl_4
// Provides: {"IPersistFile_Vtbl"}
// Dependencies: {}
# [repr (C)] pub struct IPersistFile_Vtbl { pub base__ : IPersist_Vtbl , pub IsDirty : unsafe extern "system" fn (* mut core :: ffi :: c_void) -> HRESULT , Load : usize , pub Save : unsafe extern "system" fn (* mut core :: ffi :: c_void , PCWSTR , BOOL) -> HRESULT , pub SaveCompleted : unsafe extern "system" fn (* mut core :: ffi :: c_void , PCWSTR) -> HRESULT , pub GetCurFile : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut PWSTR) -> HRESULT , }
};
}
