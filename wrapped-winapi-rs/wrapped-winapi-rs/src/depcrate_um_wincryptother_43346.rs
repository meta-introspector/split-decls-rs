// Generated macro for other_43346 (other)
macro_rules! Depcrate_um_wincryptother_43346 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_43346"}
// Dependencies: {}
extern "system" { pub fn CertSetStoreProperty (hCertStore : HCERTSTORE , dwPropId : DWORD , dwFlags : DWORD , pvData : * const c_void ,) -> BOOL ; pub fn CertGetStoreProperty (hCertStore : HCERTSTORE , dwPropId : DWORD , pvData : * mut c_void , pcbData : * mut DWORD ,) -> BOOL ; }
};
}
