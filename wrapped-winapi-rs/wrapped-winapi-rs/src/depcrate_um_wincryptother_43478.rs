// Generated macro for other_43478 (other)
macro_rules! Depcrate_um_wincryptother_43478 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_43478"}
// Dependencies: {}
extern "system" { pub fn CryptImportPKCS8 (sPrivateKeyAndParams : CRYPT_PKCS8_IMPORT_PARAMS , dwFlags : DWORD , phCryptProv : * mut HCRYPTPROV , pvAuxInfo : * mut c_void ,) -> BOOL ; }
};
}
