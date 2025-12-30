// Generated macro for other_43458 (other)
macro_rules! Depcrate_um_wincryptother_43458 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_43458"}
// Dependencies: {}
extern "system" { pub fn CryptImportPublicKeyInfoEx (hCryptProv : HCRYPTPROV , dwCertEncodingType : DWORD , pInfo : PCERT_PUBLIC_KEY_INFO , aiKeyAlg : ALG_ID , dwFlags : DWORD , pvAuxInfo : * mut c_void , phKey : * mut HCRYPTKEY ,) -> BOOL ; pub fn CryptImportPublicKeyInfoEx2 (dwCertEncodingType : DWORD , pInfo : PCERT_PUBLIC_KEY_INFO , dwFlags : DWORD , pvAuxInfo : * mut c_void , phKey : * mut BCRYPT_KEY_HANDLE ,) -> BOOL ; }
};
}
