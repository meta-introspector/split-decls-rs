// Generated macro for macro_2067 (macro)
macro_rules! Depcrate_shared_d3d9macro_2067 {
() => {
// Module: crate::shared::d3d9
// Provides: {"macro_2067"}
// Dependencies: {}
RIDL ! { # [uuid (0x85c31227 , 0x3de5 , 0x4f00 , 0x9b , 0x3a , 0xf1 , 0x1a , 0xc3 , 0x8c , 0x18 , 0xb5)] interface IDirect3DTexture9 (IDirect3DTexture9Vtbl) : IDirect3DBaseTexture9 (IDirect3DBaseTexture9Vtbl) { fn GetLevelDesc (Level : UINT , pDesc : * mut D3DSURFACE_DESC ,) -> HRESULT , fn GetSurfaceLevel (Level : UINT , ppSurfaceLevel : * mut * mut IDirect3DSurface9 ,) -> HRESULT , fn LockRect (Level : UINT , pLockedRect : * mut D3DLOCKED_RECT , pRect : * const RECT , Flags : DWORD ,) -> HRESULT , fn UnlockRect (Level : UINT ,) -> HRESULT , fn AddDirtyRect (pDirtyRect : * const RECT ,) -> HRESULT , } }
};
}
