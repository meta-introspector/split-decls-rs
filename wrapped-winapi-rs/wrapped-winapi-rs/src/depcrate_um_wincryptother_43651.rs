// Generated macro for other_43651 (other)
macro_rules! Depcrate_um_wincryptother_43651 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_43651"}
// Dependencies: {}
extern "system" { pub fn CryptRetrieveObjectByUrlA (pszUrl : LPCSTR , pszObjectOid : LPCSTR , dwRetrievalFlags : DWORD , dwTimeout : DWORD , ppvObject : * mut LPVOID , hAsyncRetrieve : HCRYPTASYNC , pCredentials : PCRYPT_CREDENTIALS , pvVerify : LPVOID , pAuxInfo : PCRYPT_RETRIEVE_AUX_INFO ,) -> BOOL ; pub fn CryptRetrieveObjectByUrlW (pszUrl : LPCWSTR , pszObjectOid : LPCSTR , dwRetrievalFlags : DWORD , dwTimeout : DWORD , ppvObject : * mut LPVOID , hAsyncRetrieve : HCRYPTASYNC , pCredentials : PCRYPT_CREDENTIALS , pvVerify : LPVOID , pAuxInfo : PCRYPT_RETRIEVE_AUX_INFO ,) -> BOOL ; }
};
}
