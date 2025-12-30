// Generated macro for macro_2082 (macro)
macro_rules! Depcrate_shared_d3d9macro_2082 {
() => {
// Module: crate::shared::d3d9
// Provides: {"macro_2082"}
// Dependencies: {}
RIDL ! { # [uuid (0xcfbaf3a , 0x9ff6 , 0x429a , 0x99 , 0xb3 , 0xa2 , 0x79 , 0x6a , 0xf8 , 0xb8 , 0x9b)] interface IDirect3DSurface9 (IDirect3DSurface9Vtbl) : IDirect3DResource9 (IDirect3DResource9Vtbl) { fn GetContainer (riid : * const IID , ppContainer : * mut * mut VOID ,) -> HRESULT , fn GetDesc (pDesc : * mut D3DSURFACE_DESC ,) -> HRESULT , fn LockRect (pLockedRect : * mut D3DLOCKED_RECT , pRect : * const RECT , Flags : DWORD ,) -> HRESULT , fn UnlockRect () -> HRESULT , fn GetDC (phdc : * mut HDC ,) -> HRESULT , fn ReleaseDC (hdc : HDC ,) -> HRESULT , } }
};
}
