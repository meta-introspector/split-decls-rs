// Generated macro for IStorage_Vtbl (struct)
macro_rules! Depcrate_b_nestedIStorage_Vtbl {
() => {
// Module: crate::b_nested
// Provides: {"IStorage_Vtbl"}
// Dependencies: {}
# [repr (C)] pub struct IStorage_Vtbl { pub base__ : IUnknown_Vtbl , CreateStream : usize , OpenStream : usize , CreateStorage : usize , OpenStorage : usize , pub CopyTo : unsafe extern "system" fn (* mut core :: ffi :: c_void , u32 , * const GUID , * const * const u16 , * mut core :: ffi :: c_void ,) -> HRESULT , MoveElementTo : usize , pub Commit : unsafe extern "system" fn (* mut core :: ffi :: c_void , u32) -> HRESULT , pub Revert : unsafe extern "system" fn (* mut core :: ffi :: c_void) -> HRESULT , EnumElements : usize , pub DestroyElement : unsafe extern "system" fn (* mut core :: ffi :: c_void , PCWSTR) -> HRESULT , pub RenameElement : unsafe extern "system" fn (* mut core :: ffi :: c_void , PCWSTR , PCWSTR) -> HRESULT , SetElementTimes : usize , pub SetClass : unsafe extern "system" fn (* mut core :: ffi :: c_void , * const GUID) -> HRESULT , pub SetStateBits : unsafe extern "system" fn (* mut core :: ffi :: c_void , u32 , u32) -> HRESULT , Stat : usize , }
};
}
