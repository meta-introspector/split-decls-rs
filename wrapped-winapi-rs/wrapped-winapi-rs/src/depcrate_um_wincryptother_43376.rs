// Generated macro for other_43376 (other)
macro_rules! Depcrate_um_wincryptother_43376 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_43376"}
// Dependencies: {}
extern "system" { pub fn CertEnumSystemStoreLocation (dwFlags : DWORD , pvArg : * mut c_void , pfnEnum : PFN_CERT_ENUM_SYSTEM_STORE_LOCATION ,) -> BOOL ; pub fn CertEnumSystemStore (dwFlags : DWORD , pvSystemStoreLocationPara : * mut c_void , pvArg : * mut c_void , pfnEnum : PFN_CERT_ENUM_SYSTEM_STORE ,) -> BOOL ; pub fn CertEnumPhysicalStore (pvSystemStore : * const c_void , dwFlags : DWORD , pvArg : * mut c_void , pfnEnum : PFN_CERT_ENUM_PHYSICAL_STORE ,) -> BOOL ; }
};
}
