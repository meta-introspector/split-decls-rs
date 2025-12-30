// Generated macro for other_44161 (other)
macro_rules! Depcrate_um_wincryptother_44161 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_44161"}
// Dependencies: {}
extern "system" { pub fn CertAddRefServerOcspResponse (hServerOcspResponse : HCERT_SERVER_OCSP_RESPONSE ,) ; pub fn CertCloseServerOcspResponse (hServerOcspResponse : HCERT_SERVER_OCSP_RESPONSE , dwFlags : DWORD ,) ; pub fn CertGetServerOcspResponseContext (hServerOcspResponse : HCERT_SERVER_OCSP_RESPONSE , dwFlags : DWORD , pvReserved : LPVOID ,) -> PCCERT_SERVER_OCSP_RESPONSE_CONTEXT ; pub fn CertAddRefServerOcspResponseContext (pServerOcspResponseContext : PCCERT_SERVER_OCSP_RESPONSE_CONTEXT ,) ; pub fn CertFreeServerOcspResponseContext (pServerOcspResponseContext : PCCERT_SERVER_OCSP_RESPONSE_CONTEXT ,) ; pub fn CertRetrieveLogoOrBiometricInfo (pCertContext : PCCERT_CONTEXT , lpszLogoOrBiometricType : LPCSTR , dwRetrievalFlags : DWORD , dwTimeout : DWORD , dwFlags : DWORD , pvReserved : * mut c_void , ppbData : * mut * mut BYTE , pcbData : * mut DWORD , ppwszMimeType : * mut LPWSTR ,) -> BOOL ; }
};
}
