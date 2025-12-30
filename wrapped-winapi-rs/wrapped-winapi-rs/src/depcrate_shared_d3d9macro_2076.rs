// Generated macro for macro_2076 (macro)
macro_rules! Depcrate_shared_d3d9macro_2076 {
() => {
// Module: crate::shared::d3d9
// Provides: {"macro_2076"}
// Dependencies: {}
RIDL ! { # [uuid (0xb64bb1b5 , 0xfd70 , 0x4df6 , 0xbf , 0x91 , 0x19 , 0xd0 , 0xa1 , 0x24 , 0x55 , 0xe3)] interface IDirect3DVertexBuffer9 (IDirect3DVertexBuffer9Vtbl) : IDirect3DResource9 (IDirect3DResource9Vtbl) { fn Lock (OffsetToLock : UINT , SizeToLock : UINT , ppbData : * mut * mut VOID , Flags : DWORD ,) -> HRESULT , fn Unlock () -> HRESULT , fn GetDesc (pDesc : * mut D3DVERTEXBUFFER_DESC ,) -> HRESULT , } }
};
}
