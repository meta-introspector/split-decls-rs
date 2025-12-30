// Generated macro for other_43653 (other)
macro_rules! Depcrate_um_wincryptother_43653 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_43653"}
// Dependencies: {}
extern "system" { pub fn CryptInstallCancelRetrieval (pfnCancel : PFN_CRYPT_CANCEL_RETRIEVAL , pvArg : * const c_void , dwFlags : DWORD , pvReserved : * mut c_void ,) -> BOOL ; pub fn CryptUninstallCancelRetrieval (dwFlags : DWORD , pvReserved : * mut c_void ,) -> BOOL ; pub fn CryptCancelAsyncRetrieval (hAsyncRetrieval : HCRYPTASYNC ,) -> BOOL ; }
};
}
