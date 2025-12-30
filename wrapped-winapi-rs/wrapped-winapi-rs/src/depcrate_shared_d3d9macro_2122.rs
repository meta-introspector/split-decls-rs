// Generated macro for macro_2122 (macro)
macro_rules! Depcrate_shared_d3d9macro_2122 {
() => {
// Module: crate::shared::d3d9
// Provides: {"macro_2122"}
// Dependencies: {}
RIDL ! { # [uuid (0xff24beee , 0xda21 , 0x4beb , 0x98 , 0xb5 , 0xd2 , 0xf8 , 0x99 , 0xf9 , 0x8a , 0xf9)] interface IDirect3DAuthenticatedChannel9 (IDirect3DAuthenticatedChannel9Vtbl) : IUnknown (IUnknownVtbl) { fn GetCertificateSize (pCertificateSize : * mut UINT ,) -> HRESULT , fn GetCertificate (CertifacteSize : UINT , ppCertificate : * mut BYTE ,) -> HRESULT , fn NegotiateKeyExchange (DataSize : UINT , pData : * mut VOID ,) -> HRESULT , fn Query (InputSize : UINT , pInput : * const VOID , OutputSize : UINT , pOutput : * mut VOID ,) -> HRESULT , fn Configure (InputSize : UINT , pInput : * const VOID , pOutput : * mut D3DAUTHENTICATEDCHANNEL_CONFIGURE_OUTPUT ,) -> HRESULT , } }
};
}
