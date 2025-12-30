// Generated macro for other_43989 (other)
macro_rules! Depcrate_um_wincryptother_43989 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_43989"}
// Dependencies: {}
extern "system" { pub fn CertGetCertificateChain (hChainEngine : HCERTCHAINENGINE , pCertContext : PCCERT_CONTEXT , pTime : LPFILETIME , hAdditionalStore : HCERTSTORE , pChainPara : PCERT_CHAIN_PARA , dwFlags : DWORD , pvReserved : LPVOID , ppChainContext : * mut PCCERT_CHAIN_CONTEXT ,) -> BOOL ; pub fn CertFreeCertificateChain (pChainContext : PCCERT_CHAIN_CONTEXT ,) ; pub fn CertDuplicateCertificateChain (pChainContext : PCCERT_CHAIN_CONTEXT ,) -> PCCERT_CHAIN_CONTEXT ; }
};
}
