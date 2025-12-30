// Generated macro for other_43301 (other)
macro_rules! Depcrate_um_wincryptother_43301 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_43301"}
// Dependencies: {}
extern "system" { pub fn CertDuplicateCRLContext (pCrlContext : PCCRL_CONTEXT ,) -> PCCRL_CONTEXT ; pub fn CertCreateCRLContext (dwCertEncodingType : DWORD , pbCrlEncoded : * const BYTE , cbCrlEncoded : DWORD ,) -> PCCRL_CONTEXT ; pub fn CertFreeCRLContext (pCrlContext : PCCRL_CONTEXT ,) -> BOOL ; pub fn CertSetCRLContextProperty (pCrlContext : PCCRL_CONTEXT , dwPropId : DWORD , dwFlags : DWORD , pvData : * const c_void ,) -> BOOL ; pub fn CertGetCRLContextProperty (pCrlContext : PCCRL_CONTEXT , dwPropId : DWORD , pvData : * mut c_void , pcbData : * mut DWORD ,) -> BOOL ; pub fn CertEnumCRLContextProperties (pCrlContext : PCCRL_CONTEXT , dwPropId : DWORD ,) -> DWORD ; pub fn CertFindCertificateInCRL (pCert : PCCERT_CONTEXT , pCrlContext : PCCRL_CONTEXT , dwFlags : DWORD , pvReserved : * mut c_void , ppCrlEntry : * mut PCRL_ENTRY ,) -> BOOL ; pub fn CertIsValidCRLForCertificate (pCert : PCCERT_CONTEXT , pCrl : PCCRL_CONTEXT , dwFlags : DWORD , pvReserved : * mut c_void ,) -> BOOL ; }
};
}
