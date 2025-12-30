// Generated macro for other_43453 (other)
macro_rules! Depcrate_um_wincryptother_43453 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_43453"}
// Dependencies: {}
extern "system" { pub fn CryptExportPublicKeyInfoFromBCryptKeyHandle (hBCryptKey : BCRYPT_KEY_HANDLE , dwCertEncodingType : DWORD , pszPublicKeyObjId : LPSTR , dwFlags : DWORD , pvAuxInfo : * mut c_void , pInfo : PCERT_PUBLIC_KEY_INFO , pcbInfo : * mut DWORD ,) -> BOOL ; }
};
}
