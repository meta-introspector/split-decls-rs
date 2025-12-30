// Generated macro for other_43580 (other)
macro_rules! Depcrate_um_wincryptother_43580 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_43580"}
// Dependencies: {}
extern "system" { pub fn CryptCreateAsyncHandle (dwFlags : DWORD , phAsync : PHCRYPTASYNC ,) -> BOOL ; pub fn CryptSetAsyncParam (hAsync : HCRYPTASYNC , pszParamOid : LPSTR , pvParam : LPVOID , pfnFree : PFN_CRYPT_ASYNC_PARAM_FREE_FUNC ,) -> BOOL ; pub fn CryptGetAsyncParam (hAsync : HCRYPTASYNC , pszParamOid : LPSTR , ppvParam : * mut LPVOID , ppfnFree : * mut PFN_CRYPT_ASYNC_PARAM_FREE_FUNC ,) -> BOOL ; pub fn CryptCloseAsyncHandle (hAsync : HCRYPTASYNC ,) -> BOOL ; }
};
}
