// Generated macro for other_43461 (other)
macro_rules! Depcrate_um_wincryptother_43461 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_43461"}
// Dependencies: {}
extern "system" { pub fn CryptAcquireCertificatePrivateKey (pCert : PCCERT_CONTEXT , dwFlags : DWORD , pvParameters : * mut c_void , phCryptProvOrNCryptKey : * mut HCRYPTPROV_OR_NCRYPT_KEY_HANDLE , pdwKeySpec : * mut DWORD , pfCallerFreeProvOrNCryptKey : * mut BOOL ,) -> BOOL ; }
};
}
