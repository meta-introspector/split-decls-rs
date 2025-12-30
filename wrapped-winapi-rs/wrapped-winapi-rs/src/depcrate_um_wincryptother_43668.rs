// Generated macro for other_43668 (other)
macro_rules! Depcrate_um_wincryptother_43668 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_43668"}
// Dependencies: {}
extern "system" { pub fn CryptGetObjectUrl (pszUrlOid : LPCSTR , pvPara : LPVOID , dwFlags : DWORD , pUrlArray : PCRYPT_URL_ARRAY , pcbUrlArray : * mut DWORD , pUrlInfo : PCRYPT_URL_INFO , pcbUrlInfo : * mut DWORD , pvReserved : LPVOID ,) -> BOOL ; }
};
}
