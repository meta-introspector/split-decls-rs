// Generated macro for other_42534 (other)
macro_rules! Depcrate_um_wincryptother_42534 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_42534"}
// Dependencies: {}
extern "system" { pub fn CryptEnumOIDInfo (dwGroupId : DWORD , dwFlags : DWORD , pvArg : * mut c_void , pfnEnumOIDInfo : PFN_CRYPT_ENUM_OID_INFO ,) -> BOOL ; pub fn CryptFindLocalizedName (pwszCryptName : LPCWSTR ,) -> LPCWSTR ; }
};
}
