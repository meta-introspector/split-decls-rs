// Generated macro for other_43449 (other)
macro_rules! Depcrate_um_wincryptother_43449 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_43449"}
// Dependencies: {}
extern "system" { pub fn CryptUninstallDefaultContext (hDefaultContext : HCRYPTDEFAULTCONTEXT , dwFlags : DWORD , pvReserved : * mut c_void ,) -> BOOL ; pub fn CryptExportPublicKeyInfo (hCryptProvOrNCryptKey : HCRYPTPROV_OR_NCRYPT_KEY_HANDLE , dwKeySpec : DWORD , dwCertEncodingType : DWORD , pInfo : PCERT_PUBLIC_KEY_INFO , pcbInfo : * mut DWORD ,) -> BOOL ; pub fn CryptExportPublicKeyInfoEx (hCryptProvOrNCryptKey : HCRYPTPROV_OR_NCRYPT_KEY_HANDLE , dwKeySpec : DWORD , dwCertEncodingType : DWORD , pszPublicKeyObjId : LPSTR , dwFlags : DWORD , pvAuxInfo : * mut c_void , pInfo : PCERT_PUBLIC_KEY_INFO , pcbInfo : * mut DWORD ,) -> BOOL ; }
};
}
