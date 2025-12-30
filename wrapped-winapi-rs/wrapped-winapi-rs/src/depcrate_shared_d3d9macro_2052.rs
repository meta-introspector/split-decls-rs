// Generated macro for macro_2052 (macro)
macro_rules! Depcrate_shared_d3d9macro_2052 {
() => {
// Module: crate::shared::d3d9
// Provides: {"macro_2052"}
// Dependencies: {}
RIDL ! { # [uuid (0x5eec05d , 0x8f7d , 0x4362 , 0xb9 , 0x99 , 0xd1 , 0xba , 0xf3 , 0x57 , 0xc7 , 0x4)] interface IDirect3DResource9 (IDirect3DResource9Vtbl) : IUnknown (IUnknownVtbl) { fn GetDevice (ppDevice : * mut * mut IDirect3DDevice9 ,) -> HRESULT , fn SetPrivateData (refguid : * const GUID , pData : * const VOID , SizeOfData : DWORD , Flags : DWORD ,) -> HRESULT , fn GetPrivateData (refguid : * const GUID , pData : * mut VOID , pSizeOfData : * mut DWORD ,) -> HRESULT , fn FreePrivateData (refguid : * const GUID ,) -> HRESULT , fn SetPriority (PriorityNew : DWORD ,) -> DWORD , fn GetPriority () -> DWORD , fn PreLoad () -> () , fn GetType () -> D3DRESOURCETYPE , } }
};
}
