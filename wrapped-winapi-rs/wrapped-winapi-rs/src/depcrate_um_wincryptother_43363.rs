// Generated macro for other_43363 (other)
macro_rules! Depcrate_um_wincryptother_43363 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_43363"}
// Dependencies: {}
extern "system" { pub fn CertRegisterSystemStore (pvSystemStore : * const c_void , dwFlags : DWORD , pStoreInfo : PCERT_SYSTEM_STORE_INFO , pvReserved : * mut c_void ,) -> BOOL ; pub fn CertRegisterPhysicalStore (pvSystemStore : * const c_void , dwFlags : DWORD , pwszStoreName : LPCWSTR , pStoreInfo : PCERT_PHYSICAL_STORE_INFO , pvReserved : * mut c_void ,) -> BOOL ; pub fn CertUnregisterSystemStore (pvSystemStore : * const c_void , dwFlags : DWORD ,) -> BOOL ; pub fn CertUnregisterPhysicalStore (pvSystemStore : * const c_void , dwFlags : DWORD , pwszStoreName : LPCWSTR ,) -> BOOL ; }
};
}
