// Generated macro for other_30513 (other)
macro_rules! Depcrate_um_lmjoinother_30513 {
() => {
// Module: crate::um::lmjoin
// Provides: {"other_30513"}
// Dependencies: {}
extern "system" { pub fn NetJoinDomain (lpServer : LPCWSTR , lpDomain : LPCWSTR , lpMachineAccountOU : LPCWSTR , lpAccount : LPCWSTR , lpPassword : LPCWSTR , fJoinOptions : DWORD ,) -> NET_API_STATUS ; pub fn NetUnjoinDomain (lpServer : LPCWSTR , lpAccount : LPCWSTR , lpPassword : LPCWSTR , fUnjoinOptions : DWORD ,) -> NET_API_STATUS ; pub fn NetRenameMachineInDomain (lpServer : LPCWSTR , lpNewMachineName : LPCWSTR , lpAccount : LPCWSTR , lpPassword : LPCWSTR , fRenameOptions : DWORD ,) -> NET_API_STATUS ; pub fn NetValidateName (lpServer : LPCWSTR , lpName : LPCWSTR , lpAccount : LPCWSTR , lpPassword : LPCWSTR , NameType : NETSETUP_NAME_TYPE ,) -> NET_API_STATUS ; pub fn NetGetJoinableOUs (lpServer : LPCWSTR , lpDomain : LPCWSTR , lpAccount : LPCWSTR , lpPassword : LPCWSTR , OUCount : * mut DWORD , OUs : * mut * mut LPWSTR ,) -> NET_API_STATUS ; }
};
}
