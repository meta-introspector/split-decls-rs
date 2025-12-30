// Generated macro for other_30627 (other)
macro_rules! Depcrate_um_lmreplother_30627 {
() => {
// Module: crate::um::lmrepl
// Provides: {"other_30627"}
// Dependencies: {}
extern "system" { pub fn NetReplImportDirAdd (servername : LPCWSTR , level : DWORD , buf : LPBYTE , parm_err : LPDWORD ,) -> NET_API_STATUS ; pub fn NetReplImportDirDel (servername : LPCWSTR , dirname : LPCWSTR ,) -> NET_API_STATUS ; pub fn NetReplImportDirEnum (servername : LPCWSTR , level : DWORD , bufptr : * mut LPBYTE , prefmaxlen : DWORD , entriesread : LPDWORD , totalentries : LPDWORD , resumehandle : LPDWORD ,) -> NET_API_STATUS ; pub fn NetReplImportDirGetInfo (servername : LPCWSTR , dirname : LPCWSTR , level : DWORD , bufptr : * mut LPBYTE ,) -> NET_API_STATUS ; pub fn NetReplImportDirLock (servername : LPCWSTR , dirname : LPCWSTR ,) -> NET_API_STATUS ; pub fn NetReplImportDirUnlock (servername : LPCWSTR , dirname : LPCWSTR , unlockforce : DWORD ,) -> NET_API_STATUS ; }
};
}
