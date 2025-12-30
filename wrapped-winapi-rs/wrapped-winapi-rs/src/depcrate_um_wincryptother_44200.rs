// Generated macro for other_44200 (other)
macro_rules! Depcrate_um_wincryptother_44200 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_44200"}
// Dependencies: {}
extern "system" { pub fn CertSelectCertificateChains (pSelectionContext : LPCGUID , dwFlags : DWORD , pChainParameters : PCCERT_SELECT_CHAIN_PARA , cCriteria : DWORD , rgpCriteria : PCCERT_SELECT_CRITERIA , hStore : HCERTSTORE , pcSelection : PDWORD , pprgpSelection : * mut * mut PCCERT_CHAIN_CONTEXT ,) -> BOOL ; pub fn CertFreeCertificateChainList (prgpSelection : * mut PCCERT_CHAIN_CONTEXT ,) ; }
};
}
