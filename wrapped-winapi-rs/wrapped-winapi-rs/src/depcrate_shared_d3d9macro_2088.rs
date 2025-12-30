// Generated macro for macro_2088 (macro)
macro_rules! Depcrate_shared_d3d9macro_2088 {
() => {
// Module: crate::shared::d3d9
// Provides: {"macro_2088"}
// Dependencies: {}
RIDL ! { # [uuid (0xd9771460 , 0xa695 , 0x4f26 , 0xbb , 0xd3 , 0x27 , 0xb8 , 0x40 , 0xb5 , 0x41 , 0xcc)] interface IDirect3DQuery9 (IDirect3DQuery9Vtbl) : IUnknown (IUnknownVtbl) { fn GetDevice (ppDevice : * mut * mut IDirect3DDevice9 ,) -> HRESULT , fn GetType () -> D3DRESOURCETYPE , fn GetDataSize () -> DWORD , fn Issue (dwIssueFlags : DWORD ,) -> HRESULT , fn GetData (pData : * mut VOID , dwSize : DWORD , dwGetDataFlags : DWORD ,) -> HRESULT , } }
};
}
