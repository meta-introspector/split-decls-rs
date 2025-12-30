// Generated macro for other_44228 (other)
macro_rules! Depcrate_um_wincryptother_44228 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_44228"}
// Dependencies: {}
extern "system" { pub fn CryptRetrieveTimeStamp (wszUrl : LPCWSTR , dwRetrievalFlags : DWORD , dwTimeout : DWORD , pszHashId : LPCSTR , pPara : * const CRYPT_TIMESTAMP_PARA , pbData : * const BYTE , cbData : DWORD , ppTsContext : * mut PCRYPT_TIMESTAMP_CONTEXT , ppTsSigner : * mut PCCERT_CONTEXT , phStore : * mut HCERTSTORE ,) -> BOOL ; }
};
}
