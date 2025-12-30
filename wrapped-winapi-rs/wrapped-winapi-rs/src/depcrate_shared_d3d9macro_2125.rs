// Generated macro for macro_2125 (macro)
macro_rules! Depcrate_shared_d3d9macro_2125 {
() => {
// Module: crate::shared::d3d9
// Provides: {"macro_2125"}
// Dependencies: {}
RIDL ! { # [uuid (0xfa0ab799 , 0x7a9c , 0x48ca , 0x8c , 0x5b , 0x23 , 0x7e , 0x71 , 0xa5 , 0x44 , 0x34)] interface IDirect3DCryptoSession9 (IDirect3DCryptoSession9Vtbl) : IUnknown (IUnknownVtbl) { fn GetCertificateSize (pCertificateSize : * mut UINT ,) -> HRESULT , fn GetCertificate (CertifacteSize : UINT , ppCertificate : * mut BYTE ,) -> HRESULT , fn NegotiateKeyExchange (DataSize : UINT , pData : * mut VOID ,) -> HRESULT , fn EncryptionBlt (pSrcSurface : * mut IDirect3DSurface9 , pDstSurface : * mut IDirect3DSurface9 , DstSurfaceSize : UINT , pIV : * mut VOID ,) -> HRESULT , fn DecryptionBlt (pSrcSurface : * mut IDirect3DSurface9 , pDstSurface : * mut IDirect3DSurface9 , SrcSurfaceSize : UINT , pEncryptedBlockInfo : * mut D3DENCRYPTED_BLOCK_INFO , pContentKey : * mut VOID , pIV : * mut VOID ,) -> HRESULT , fn GetSurfacePitch (pSrcSurface : * mut IDirect3DSurface9 , pSurfacePitch : * mut UINT ,) -> HRESULT , fn StartSessionKeyRefresh (pRandomNumber : * mut VOID , RandomNumberSize : UINT ,) -> HRESULT , fn FinishSessionKeyRefresh () -> HRESULT , fn GetEncryptionBltKey (pReadbackKey : * mut VOID , KeySize : UINT ,) -> HRESULT , } }
};
}
