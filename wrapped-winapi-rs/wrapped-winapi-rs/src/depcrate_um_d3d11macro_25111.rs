// Generated macro for macro_25111 (macro)
macro_rules! Depcrate_um_d3d11macro_25111 {
() => {
// Module: crate::um::d3d11
// Provides: {"macro_25111"}
// Dependencies: {}
RIDL ! { # [uuid (0x3015a308 , 0xdcbd , 0x47aa , 0xa7 , 0x47 , 0x19 , 0x24 , 0x86 , 0xd1 , 0x4d , 0x4a)] interface ID3D11AuthenticatedChannel (ID3D11AuthenticatedChannelVtbl) : ID3D11DeviceChild (ID3D11DeviceChildVtbl) { fn GetCertificateSize (pCertificateSize : * mut UINT ,) -> HRESULT , fn GetCertificate (CertificateSize : UINT , pCertificate : * mut BYTE ,) -> HRESULT , fn GetChannelHandle (pChannelHandle : * mut HANDLE ,) -> () , } }
};
}
