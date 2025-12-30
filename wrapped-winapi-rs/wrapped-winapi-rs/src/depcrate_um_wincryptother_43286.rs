// Generated macro for other_43286 (other)
macro_rules! Depcrate_um_wincryptother_43286 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_43286"}
// Dependencies: {}
extern "system" { pub fn CertGetIssuerCertificateFromStore (hCertStore : HCERTSTORE , pSubjectContext : PCCERT_CONTEXT , pPrevIssuerContext : PCCERT_CONTEXT , pdwFlags : * mut DWORD ,) -> PCCERT_CONTEXT ; pub fn CertVerifySubjectCertificateContext (pSubject : PCCERT_CONTEXT , pIssuer : PCCERT_CONTEXT , pdwFlags : * mut DWORD ,) -> BOOL ; pub fn CertDuplicateCertificateContext (pCertContext : PCCERT_CONTEXT ,) -> PCCERT_CONTEXT ; pub fn CertCreateCertificateContext (dwCertEncodingType : DWORD , pbCertEncoded : * const BYTE , cbCertEncoded : DWORD ,) -> PCCERT_CONTEXT ; pub fn CertFreeCertificateContext (pCertContext : PCCERT_CONTEXT ,) -> BOOL ; pub fn CertSetCertificateContextProperty (pCertContext : PCCERT_CONTEXT , dwPropId : DWORD , dwFlags : DWORD , pvData : * const c_void ,) -> BOOL ; }
};
}
