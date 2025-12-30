// Generated macro for other_30024 (other)
macro_rules! Depcrate_um_lmaccessother_30024 {
() => {
// Module: crate::um::lmaccess
// Provides: {"other_30024"}
// Dependencies: {}
extern "system" { pub fn NetRemoveServiceAccount (ServerName : LPWSTR , AccountName : LPWSTR , Flags : DWORD ,) -> NTSTATUS ; pub fn NetEnumerateServiceAccounts (ServerName : LPWSTR , Flags : DWORD , AccountsCount : * mut DWORD , Accounts : * mut PZPWSTR ,) -> NTSTATUS ; pub fn NetIsServiceAccount (ServerName : LPWSTR , AccountName : LPWSTR , IsService : * mut BOOL ,) -> NTSTATUS ; pub fn NetQueryServiceAccount (ServerName : LPWSTR , AccountName : LPWSTR , InfoLevel : DWORD , Buffer : * mut PBYTE ,) -> NTSTATUS ; }
};
}
