// Generated macro for other_42475 (other)
macro_rules! Depcrate_um_wincryptother_42475 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_42475"}
// Dependencies: {}
extern "system" { pub fn CryptInstallOIDFunctionAddress (hModule : HMODULE , dwEncodingType : DWORD , pszFuncName : LPCSTR , cFuncEntry : DWORD , rgFuncEntry : * const CRYPT_OID_FUNC_ENTRY , dwFlags : DWORD ,) -> BOOL ; pub fn CryptInitOIDFunctionSet (pszFuncName : LPCSTR , dwFlags : DWORD ,) -> HCRYPTOIDFUNCSET ; pub fn CryptGetOIDFunctionAddress (hFuncSet : HCRYPTOIDFUNCSET , dwEncodingType : DWORD , pszOID : LPCSTR , dwFlags : DWORD , ppvFuncAddr : * mut * mut c_void , phFuncAddr : * mut HCRYPTOIDFUNCADDR ,) -> BOOL ; }
};
}
