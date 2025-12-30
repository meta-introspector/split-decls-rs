// Generated macro for other_44121 (other)
macro_rules! Depcrate_um_wincryptother_44121 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_44121"}
// Dependencies: {}
extern "system" { pub fn PFXImportCertStore (pPFX : * mut CRYPT_DATA_BLOB , szPassword : LPCWSTR , dwFlags : DWORD ,) -> HCERTSTORE ; }
};
}
