// Generated macro for other_44232 (other)
macro_rules! Depcrate_um_wincryptother_44232 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_44232"}
// Dependencies: {}
extern "system" { pub fn CryptVerifyTimeStampSignature (pbTSContentInfo : * const BYTE , cbTSContentInfo : DWORD , pbData : * const BYTE , cbData : DWORD , hAdditionalStore : HCERTSTORE , ppTsContext : * mut PCRYPT_TIMESTAMP_CONTEXT , ppTsSigner : * mut PCCERT_CONTEXT , phStore : * mut HCERTSTORE ,) -> BOOL ; }
};
}
