// Generated macro for other_44253 (other)
macro_rules! Depcrate_um_wincryptother_44253 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_44253"}
// Dependencies: {}
extern "system" { pub fn CertIsWeakHash (dwHashUseType : DWORD , pwszCNGHashAlgid : LPCWSTR , dwChainFlags : DWORD , pSignerChainContext : PCCERT_CHAIN_CONTEXT , pTimeStamp : LPFILETIME , pwszFileName : LPCWSTR ,) -> BOOL ; }
};
}
