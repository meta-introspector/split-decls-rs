// Generated macro for other_42480 (other)
macro_rules! Depcrate_um_wincryptother_42480 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_42480"}
// Dependencies: {}
extern "system" { pub fn CryptUnregisterDefaultOIDFunction (dwEncodingType : DWORD , pszFuncName : LPCSTR , pwszDll : LPCWSTR ,) -> BOOL ; pub fn CryptSetOIDFunctionValue (dwEncodingType : DWORD , pszFuncName : LPCSTR , pszOID : LPCSTR , pwszValueName : LPCWSTR , dwValueType : DWORD , pbValueData : * const BYTE , cbValueData : DWORD ,) -> BOOL ; pub fn CryptGetOIDFunctionValue (dwEncodingType : DWORD , pszFuncName : LPCSTR , pszOID : LPCSTR , pwszValueName : LPCWSTR , pdwValueType : * mut DWORD , pbValueData : * mut BYTE , pcbValueData : * mut DWORD ,) -> BOOL ; }
};
}
