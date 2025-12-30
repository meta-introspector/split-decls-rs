// Generated macro for other_43309 (other)
macro_rules! Depcrate_um_wincryptother_43309 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_43309"}
// Dependencies: {}
extern "system" { pub fn CertAddEncodedCertificateToStore (hCertStore : HCERTSTORE , dwCertEncodingType : DWORD , pbCertEncoded : * const BYTE , cbCertEncoded : DWORD , dwAddDisposition : DWORD , ppCertContext : * mut PCCERT_CONTEXT ,) -> BOOL ; pub fn CertAddCertificateContextToStore (hCertStore : HCERTSTORE , pCertContext : PCCERT_CONTEXT , dwAddDisposition : DWORD , ppStoreContext : * mut PCCERT_CONTEXT ,) -> BOOL ; }
};
}
