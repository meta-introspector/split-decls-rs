// Generated macro for macro_25161 (macro)
macro_rules! Depcrate_um_d3d11macro_25161 {
() => {
// Module: crate::um::d3d11
// Provides: {"macro_25161"}
// Dependencies: {}
RIDL ! { # [uuid (0x9b32f9ad , 0xbdcc , 0x40a6 , 0xa3 , 0x9d , 0xd5 , 0xc8 , 0x65 , 0x84 , 0x57 , 0x20)] interface ID3D11CryptoSession (ID3D11CryptoSessionVtbl) : ID3D11DeviceChild (ID3D11DeviceChildVtbl) { fn GetCryptoType (pCryptoType : * mut GUID ,) -> () , fn GetDecoderProfile (pDecoderProfile : * mut GUID ,) -> () , fn GetCertificateSize (pCertificateSize : * mut UINT ,) -> HRESULT , fn GetCertificate (CertificateSize : UINT , pCertificate : * mut BYTE ,) -> HRESULT , fn GetCryptoSessionHandle (pCertificate : * mut HANDLE ,) -> () , } }
};
}
