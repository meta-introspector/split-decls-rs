// Generated macro for other_43901 (other)
macro_rules! Depcrate_um_wincryptother_43901 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_43901"}
// Dependencies: {}
extern "system" { pub fn CertCreateCertificateChainEngine (pConfig : PCERT_CHAIN_ENGINE_CONFIG , phChainEngine : * mut HCERTCHAINENGINE ,) -> BOOL ; pub fn CertFreeCertificateChainEngine (hChainEngine : HCERTCHAINENGINE ,) ; pub fn CertResyncCertificateChainEngine (hChainEngine : HCERTCHAINENGINE ,) -> BOOL ; }
};
}
