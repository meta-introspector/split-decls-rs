// Generated macro for macro_2070 (macro)
macro_rules! Depcrate_shared_d3d9macro_2070 {
() => {
// Module: crate::shared::d3d9
// Provides: {"macro_2070"}
// Dependencies: {}
RIDL ! { # [uuid (0x2518526c , 0xe789 , 0x4111 , 0xa7 , 0xb9 , 0x47 , 0xef , 0x32 , 0x8d , 0x13 , 0xe6)] interface IDirect3DVolumeTexture9 (IDirect3DVolumeTexture9Vtbl) : IDirect3DBaseTexture9 (IDirect3DBaseTexture9Vtbl) { fn GetLevelDesc (Level : UINT , pDesc : * mut D3DVOLUME_DESC ,) -> HRESULT , fn GetVolumeLevel (Level : UINT , ppVolumeLevel : * mut * mut IDirect3DVolume9 ,) -> HRESULT , fn LockBox (Level : UINT , pLockedVolume : * mut D3DLOCKED_BOX , pBox : * const D3DBOX , Flags : DWORD ,) -> HRESULT , fn UnlockBox (Level : UINT ,) -> HRESULT , fn AddDirtyBox (pDirtyBox : * const D3DBOX ,) -> HRESULT , } }
};
}
