// Generated macro for other_43994 (other)
macro_rules! Depcrate_um_wincryptother_43994 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_43994"}
// Dependencies: {}
extern "system" { pub fn CertFindChainInStore (hCertStore : HCERTSTORE , dwCertEncodingType : DWORD , dwFindFlags : DWORD , dwFindType : DWORD , pvFindPara : * const c_void , pPrevChainContext : PCCERT_CHAIN_CONTEXT ,) -> PCCERT_CHAIN_CONTEXT ; }
};
}
