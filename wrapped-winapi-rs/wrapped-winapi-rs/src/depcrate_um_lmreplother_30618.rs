// Generated macro for other_30618 (other)
macro_rules! Depcrate_um_lmreplother_30618 {
() => {
// Module: crate::um::lmrepl
// Provides: {"other_30618"}
// Dependencies: {}
extern "system" { pub fn NetReplExportDirAdd (servername : LPCWSTR , level : DWORD , buf : LPBYTE , parm_err : LPDWORD ,) -> NET_API_STATUS ; pub fn NetReplExportDirDel (servername : LPCWSTR , dirname : LPCWSTR ,) -> NET_API_STATUS ; pub fn NetReplExportDirEnum (servername : LPCWSTR , level : DWORD , bufptr : * mut LPBYTE , prefmaxlen : DWORD , entriesread : LPDWORD , totalentries : LPDWORD , resumehandle : LPDWORD ,) -> NET_API_STATUS ; pub fn NetReplExportDirGetInfo (servername : LPCWSTR , dirname : LPCWSTR , level : DWORD , bufptr : * mut LPBYTE ,) -> NET_API_STATUS ; pub fn NetReplExportDirSetInfo (servername : LPCWSTR , dirname : LPCWSTR , level : DWORD , buf : LPBYTE , parm_err : LPDWORD ,) -> NET_API_STATUS ; pub fn NetReplExportDirLock (servername : LPCWSTR , dirname : LPCWSTR ,) -> NET_API_STATUS ; pub fn NetReplExportDirUnlock (servername : LPCWSTR , dirname : LPCWSTR , unlockforce : DWORD ,) -> NET_API_STATUS ; }
};
}
