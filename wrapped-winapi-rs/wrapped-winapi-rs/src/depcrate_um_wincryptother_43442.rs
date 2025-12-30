// Generated macro for other_43442 (other)
macro_rules! Depcrate_um_wincryptother_43442 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_43442"}
// Dependencies: {}
extern "system" { pub fn CryptInstallDefaultContext (hCryptProv : HCRYPTPROV , dwDefaultType : DWORD , pvDefaultPara : * const c_void , dwFlags : DWORD , pvReserved : * mut c_void , phDefaultContext : * mut HCRYPTDEFAULTCONTEXT ,) -> BOOL ; }
};
}
