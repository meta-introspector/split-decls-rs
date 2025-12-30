// Generated macro for other_43217 (other)
macro_rules! Depcrate_um_wincryptother_43217 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_43217"}
// Dependencies: {}
extern "system" { pub fn CertCloseStore (hCertStore : HCERTSTORE , dwFlags : DWORD ,) -> BOOL ; pub fn CertGetSubjectCertificateFromStore (hCertStore : HCERTSTORE , dwCertEncodingType : DWORD , pCertId : PCERT_INFO ,) -> PCCERT_CONTEXT ; pub fn CertEnumCertificatesInStore (hCertStore : HCERTSTORE , pPrevCertContext : PCCERT_CONTEXT ,) -> PCCERT_CONTEXT ; pub fn CertFindCertificateInStore (hCertStore : HCERTSTORE , dwCertEncodingType : DWORD , dwFindFlags : DWORD , dwFindType : DWORD , pvFindPara : * const c_void , pPrevCertContext : PCCERT_CONTEXT ,) -> PCCERT_CONTEXT ; }
};
}
