// Generated macro for other_30204 (other)
macro_rules! Depcrate_um_lmdfsother_30204 {
() => {
// Module: crate::um::lmdfs
// Provides: {"other_30204"}
// Dependencies: {}
extern "system" { pub fn NetDfsAddStdRoot (ServerName : LPWSTR , RootShare : LPWSTR , Comment : LPWSTR , Flags : DWORD ,) -> NET_API_STATUS ; pub fn NetDfsRemoveStdRoot (ServerName : LPWSTR , RootShare : LPWSTR , Flags : DWORD ,) -> NET_API_STATUS ; pub fn NetDfsAddFtRoot (ServerName : LPWSTR , RootShare : LPWSTR , FtDfsName : LPWSTR , Comment : LPWSTR , Flags : DWORD ,) -> NET_API_STATUS ; pub fn NetDfsRemoveFtRoot (ServerName : LPWSTR , RootShare : LPWSTR , FtDfsName : LPWSTR , Flags : DWORD ,) -> NET_API_STATUS ; pub fn NetDfsRemoveFtRootForced (DomainName : LPWSTR , ServerName : LPWSTR , RootShare : LPWSTR , FtDfsName : LPWSTR , Flags : DWORD ,) -> NET_API_STATUS ; }
};
}
