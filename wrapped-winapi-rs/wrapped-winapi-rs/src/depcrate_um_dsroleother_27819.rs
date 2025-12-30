// Generated macro for other_27819 (other)
macro_rules! Depcrate_um_dsroleother_27819 {
() => {
// Module: crate::um::dsrole
// Provides: {"other_27819"}
// Dependencies: {}
extern "system" { pub fn DsRoleGetPrimaryDomainInformation (lpServer : LPCWSTR , InfoLevel : DSROLE_PRIMARY_DOMAIN_INFO_LEVEL , Buffer : * mut PBYTE ,) -> DWORD ; pub fn DsRoleFreeMemory (Buffer : PVOID ,) ; }
};
}
