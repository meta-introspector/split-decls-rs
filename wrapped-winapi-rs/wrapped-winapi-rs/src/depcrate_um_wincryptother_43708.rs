// Generated macro for other_43708 (other)
macro_rules! Depcrate_um_wincryptother_43708 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_43708"}
// Dependencies: {}
extern "system" { pub fn CryptSetKeyIdentifierProperty (pKeyIdentifier : * const CRYPT_HASH_BLOB , dwPropId : DWORD , dwFlags : DWORD , pwszComputerName : LPCWSTR , pvReserved : * mut c_void , pvData : * const c_void ,) -> BOOL ; }
};
}
