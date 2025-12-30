// Generated macro for macro_2119 (macro)
macro_rules! Depcrate_shared_d3d9macro_2119 {
() => {
// Module: crate::shared::d3d9
// Provides: {"macro_2119"}
// Dependencies: {}
RIDL ! { # [uuid (0x26dc4561 , 0xa1ee , 0x4ae7 , 0x96 , 0xda , 0x11 , 0x8a , 0x36 , 0xc0 , 0xec , 0x95)] interface IDirect3DDevice9Video (IDirect3DDevice9VideoVtbl) : IUnknown (IUnknownVtbl) { fn GetContentProtectionCaps (pCryptoType : * const GUID , pDecodeProfile : * const GUID , pCaps : * mut D3DCONTENTPROTECTIONCAPS ,) -> HRESULT , fn CreateAuthenticatedChannel (ChannelType : D3DAUTHENTICATEDCHANNELTYPE , ppAuthenticatedChannel : * mut * mut IDirect3DAuthenticatedChannel9 , pChannelHandle : * mut HANDLE ,) -> HRESULT , fn CreateCryptoSession (pCryptoType : * const GUID , pDecodeProfile : * const GUID , ppCryptoSession : * mut * mut IDirect3DCryptoSession9 , pCryptoHandle : * mut HANDLE ,) -> HRESULT , } }
};
}
