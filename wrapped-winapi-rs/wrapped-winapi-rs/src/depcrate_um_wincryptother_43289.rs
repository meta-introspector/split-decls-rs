// Generated macro for other_43289 (other)
macro_rules! Depcrate_um_wincryptother_43289 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_43289"}
// Dependencies: {}
extern "system" { pub fn CertGetCertificateContextProperty (pCertContext : PCCERT_CONTEXT , dwPropId : DWORD , pvData : * mut c_void , pcbData : * mut DWORD ,) -> BOOL ; pub fn CertEnumCertificateContextProperties (pCertContext : PCCERT_CONTEXT , dwPropId : DWORD ,) -> DWORD ; pub fn CertCreateCTLEntryFromCertificateContextProperties (pCertContext : PCCERT_CONTEXT , cOptAttr : DWORD , rgOptAttr : PCRYPT_ATTRIBUTE , dwFlags : DWORD , pvReserved : * mut c_void , pCtlEntry : PCTL_ENTRY , pcbCtlEntry : * mut DWORD ,) -> BOOL ; pub fn CertSetCertificateContextPropertiesFromCTLEntry (pCertContext : PCCERT_CONTEXT , pCtlEntry : PCTL_ENTRY , dwFlags : DWORD ,) -> BOOL ; pub fn CertGetCRLFromStore (hCertStore : HCERTSTORE , pIssuerContext : PCCERT_CONTEXT , pPrevCrlContext : PCCRL_CONTEXT , pdwFlags : * mut DWORD ,) -> PCCRL_CONTEXT ; pub fn CertEnumCRLsInStore (hCertStore : HCERTSTORE , pPrevCrlContext : PCCRL_CONTEXT ,) -> PCCRL_CONTEXT ; pub fn CertFindCRLInStore (hCertStore : HCERTSTORE , dwCertEncodingType : DWORD , dwFindFlags : DWORD , dwFindType : DWORD , pvFindPara : * const c_void , pPrevCrlContext : PCCRL_CONTEXT ,) -> PCCRL_CONTEXT ; }
};
}
