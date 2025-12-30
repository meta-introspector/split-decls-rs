// Generated macro for macro_2079 (macro)
macro_rules! Depcrate_shared_d3d9macro_2079 {
() => {
// Module: crate::shared::d3d9
// Provides: {"macro_2079"}
// Dependencies: {}
RIDL ! { # [uuid (0x7c9dd65e , 0xd3f7 , 0x4529 , 0xac , 0xee , 0x78 , 0x58 , 0x30 , 0xac , 0xde , 0x35)] interface IDirect3DIndexBuffer9 (IDirect3DIndexBuffer9Vtbl) : IDirect3DResource9 (IDirect3DResource9Vtbl) { fn Lock (OffsetToLock : UINT , SizeToLock : UINT , ppbData : * mut * mut VOID , Flags : DWORD ,) -> HRESULT , fn Unlock () -> HRESULT , fn GetDesc (pDesc : * mut D3DINDEXBUFFER_DESC ,) -> HRESULT , } }
};
}
