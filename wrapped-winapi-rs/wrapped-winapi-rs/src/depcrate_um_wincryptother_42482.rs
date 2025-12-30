// Generated macro for other_42482 (other)
macro_rules! Depcrate_um_wincryptother_42482 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_42482"}
// Dependencies: {}
extern "system" { pub fn CryptEnumOIDFunction (dwEncodingType : DWORD , pszFuncName : LPCSTR , pszOID : LPCSTR , dwFlags : DWORD , pvArg : * mut c_void , pfnEnumOIDFunc : PFN_CRYPT_ENUM_OID_FUNC ,) -> BOOL ; }
};
}
