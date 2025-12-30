// Generated macro for other_43410 (other)
macro_rules! Depcrate_um_wincryptother_43410 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_43410"}
// Dependencies: {}
extern "system" { pub fn CertVerifyRevocation (dwEncodingType : DWORD , dwRevType : DWORD , cContext : DWORD , rgpvContext : * mut PVOID , dwFlags : DWORD , pRevPara : PCERT_REVOCATION_PARA , pRevStatus : PCERT_REVOCATION_STATUS ,) -> BOOL ; }
};
}
