// Generated macro for other_42477 (other)
macro_rules! Depcrate_um_wincryptother_42477 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_42477"}
// Dependencies: {}
extern "system" { pub fn CryptGetDefaultOIDDllList (hFuncSet : HCRYPTOIDFUNCSET , dwEncodingType : DWORD , pwszDllList : * mut WCHAR , pcchDllList : * mut DWORD ,) -> BOOL ; pub fn CryptGetDefaultOIDFunctionAddress (hFuncSet : HCRYPTOIDFUNCSET , dwEncodingType : DWORD , pwszDll : LPCWSTR , dwFlags : DWORD , ppvFuncAddr : * mut * mut c_void , phFuncAddr : * mut HCRYPTOIDFUNCADDR ,) -> BOOL ; pub fn CryptFreeOIDFunctionAddress (hFuncAddr : HCRYPTOIDFUNCADDR , dwFlags : DWORD ,) -> BOOL ; pub fn CryptRegisterOIDFunction (dwEncodingType : DWORD , pszFuncName : LPCSTR , pszOID : LPCSTR , pwszDll : LPCWSTR , pszOverrideFuncName : LPCSTR ,) -> BOOL ; pub fn CryptUnregisterOIDFunction (dwEncodingType : DWORD , pszFuncName : LPCSTR , pszOID : LPCSTR ,) -> BOOL ; pub fn CryptRegisterDefaultOIDFunction (dwEncodingType : DWORD , pszFuncName : LPCSTR , dwIndex : DWORD , pwszDll : LPCWSTR ,) -> BOOL ; }
};
}
