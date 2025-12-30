// Generated macro for macro_2085 (macro)
macro_rules! Depcrate_shared_d3d9macro_2085 {
() => {
// Module: crate::shared::d3d9
// Provides: {"macro_2085"}
// Dependencies: {}
RIDL ! { # [uuid (0x24f416e6 , 0x1f67 , 0x4aa7 , 0xb8 , 0x8e , 0xd3 , 0x3f , 0x6f , 0x31 , 0x28 , 0xa1)] interface IDirect3DVolume9 (IDirect3DVolume9Vtbl) : IUnknown (IUnknownVtbl) { fn GetDevice (ppDevice : * mut * mut IDirect3DDevice9 ,) -> HRESULT , fn SetPrivateData (refguid : * const GUID , pData : * const VOID , SizeOfData : DWORD , Flags : DWORD ,) -> HRESULT , fn GetPrivateData (refguid : * const GUID , pData : * mut VOID , pSizeOfData : * mut DWORD ,) -> HRESULT , fn FreePrivateData (refguid : * const GUID ,) -> HRESULT , fn GetContainer (riid : * const IID , ppContainer : * mut * mut VOID ,) -> HRESULT , fn GetDesc (pDesc : * mut D3DVOLUME_DESC ,) -> HRESULT , fn LockBox (pLockedVolume : * mut D3DLOCKED_BOX , pBox : * const D3DBOX , Flags : DWORD ,) -> HRESULT , fn UnlockBox () -> HRESULT , } }
};
}
