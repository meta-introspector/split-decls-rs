// Generated macro for macro_2073 (macro)
macro_rules! Depcrate_shared_d3d9macro_2073 {
() => {
// Module: crate::shared::d3d9
// Provides: {"macro_2073"}
// Dependencies: {}
RIDL ! { # [uuid (0xfff32f81 , 0xd953 , 0x473a , 0x92 , 0x23 , 0x93 , 0xd6 , 0x52 , 0xab , 0xa9 , 0x3f)] interface IDirect3DCubeTexture9 (IDirect3DCubeTexture9Vtbl) : IDirect3DBaseTexture9 (IDirect3DBaseTexture9Vtbl) { fn GetLevelDesc (Level : UINT , pDesc : * mut D3DSURFACE_DESC ,) -> HRESULT , fn GetCubeMapSurface (FaceType : D3DCUBEMAP_FACES , Level : UINT , ppCubeMapSurface : * mut * mut IDirect3DSurface9 ,) -> HRESULT , fn LockRect (FaceType : D3DCUBEMAP_FACES , Level : UINT , pLockedRect : * mut D3DLOCKED_RECT , pRect : * const RECT , Flags : DWORD ,) -> HRESULT , fn UnlockRect (FaceType : D3DCUBEMAP_FACES , Level : UINT ,) -> HRESULT , fn AddDirtyRect (FaceType : D3DCUBEMAP_FACES , pDirtyRect : * const RECT ,) -> HRESULT , } }
};
}
