// Generated macro for other_44135 (other)
macro_rules! Depcrate_um_wincryptother_44135 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_44135"}
// Dependencies: {}
extern "system" { pub fn PFXIsPFXBlob (pPFX : * mut CRYPT_DATA_BLOB ,) -> BOOL ; pub fn PFXVerifyPassword (pPFX : * mut CRYPT_DATA_BLOB , szPassword : LPCWSTR , dwFlags : DWORD ,) -> BOOL ; pub fn PFXExportCertStoreEx (hStore : HCERTSTORE , pPFX : * mut CRYPT_DATA_BLOB , szPassword : LPCWSTR , pvPara : * mut c_void , dwFlags : DWORD ,) -> BOOL ; }
};
}
