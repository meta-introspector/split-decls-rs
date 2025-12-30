// Generated macro for other_30215 (other)
macro_rules! Depcrate_um_lmdfsother_30215 {
() => {
// Module: crate::um::lmdfs
// Provides: {"other_30215"}
// Dependencies: {}
extern "system" { pub fn NetDfsRemove (DfsEntryPath : LPWSTR , ServerName : LPWSTR , ShareName : LPWSTR ,) -> NET_API_STATUS ; pub fn NetDfsEnum (DfsName : LPWSTR , Level : DWORD , PrefMaxLen : DWORD , Buffer : * mut LPBYTE , EntriesRead : LPDWORD , ResumeHandle : LPDWORD ,) -> NET_API_STATUS ; pub fn NetDfsGetInfo (DfsEntryPath : LPWSTR , ServerName : LPWSTR , ShareName : LPWSTR , Level : DWORD , Buffer : * mut LPBYTE ,) -> NET_API_STATUS ; pub fn NetDfsSetInfo (DfsEntryPath : LPWSTR , ServerName : LPWSTR , ShareName : LPWSTR , Level : DWORD , Buffer : LPBYTE ,) -> NET_API_STATUS ; pub fn NetDfsGetClientInfo (DfsEntryPath : LPWSTR , ServerName : LPWSTR , ShareName : LPWSTR , Level : DWORD , Buffer : * mut LPBYTE ,) -> NET_API_STATUS ; pub fn NetDfsSetClientInfo (DfsEntryPath : LPWSTR , ServerName : LPWSTR , ShareName : LPWSTR , Level : DWORD , Buffer : LPBYTE ,) -> NET_API_STATUS ; pub fn NetDfsMove (OldDfsEntryPath : LPWSTR , NewDfsEntryPath : LPWSTR , Flags : ULONG ,) -> NET_API_STATUS ; }
};
}
