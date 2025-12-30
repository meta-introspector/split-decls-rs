// Generated macro for macro_43962 (macro)
macro_rules! Depcrate_um_wincryptmacro_43962 {
() => {
// Module: crate::um::wincrypt
// Provides: {"macro_43962"}
// Dependencies: {}
STRUCT ! { struct CERT_CHAIN_CONTEXT { cbSize : DWORD , TrustStatus : CERT_TRUST_STATUS , cChain : DWORD , rgpChain : * mut PCERT_SIMPLE_CHAIN , cLowerQualityChainContext : DWORD , rgpLowerQualityChainContext : * mut PCCERT_CHAIN_CONTEXT , fHasRevocationFreshnessTime : BOOL , dwRevocationFreshnessTime : DWORD , dwCreateFlags : DWORD , ChainId : GUID , } }
};
}
