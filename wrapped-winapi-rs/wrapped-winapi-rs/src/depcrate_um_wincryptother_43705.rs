// Generated macro for other_43705 (other)
macro_rules! Depcrate_um_wincryptother_43705 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_43705"}
// Dependencies: {}
extern "system" { pub fn CryptGetKeyIdentifierProperty (pKeyIdentifier : * const CRYPT_HASH_BLOB , dwPropId : DWORD , dwFlags : DWORD , pwszComputerName : LPCWSTR , pvReserved : * mut c_void , pvData : * mut c_void , pcbData : * mut DWORD ,) -> BOOL ; }
};
}
