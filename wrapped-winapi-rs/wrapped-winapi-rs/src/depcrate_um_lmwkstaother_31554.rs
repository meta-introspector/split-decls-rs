// Generated macro for other_31554 (other)
macro_rules! Depcrate_um_lmwkstaother_31554 {
() => {
// Module: crate::um::lmwksta
// Provides: {"other_31554"}
// Dependencies: {}
extern "system" { pub fn NetWkstaGetInfo (servername : LMSTR , level : DWORD , bufptr : * mut LPBYTE ,) -> NET_API_STATUS ; pub fn NetWkstaSetInfo (servername : LMSTR , level : DWORD , buffer : LPBYTE , parm_err : LPDWORD ,) -> NET_API_STATUS ; pub fn NetWkstaUserGetInfo (reserved : LMSTR , level : DWORD , bufptr : * mut LPBYTE ,) -> NET_API_STATUS ; pub fn NetWkstaUserSetInfo (reserved : LMSTR , level : DWORD , buf : LPBYTE , parm_err : LPDWORD ,) -> NET_API_STATUS ; pub fn NetWkstaUserEnum (servername : LMSTR , level : DWORD , bufptr : * mut LPBYTE , prefmaxlen : DWORD , entriesread : LPDWORD , totalentries : LPDWORD , resumehandle : LPDWORD ,) -> NET_API_STATUS ; pub fn NetWkstaTransportAdd (servername : LPCWSTR , level : DWORD , buf : LPBYTE , parm_err : LPDWORD ,) -> NET_API_STATUS ; pub fn NetWkstaTransportDel (servername : LMSTR , transportname : LMSTR , ucond : DWORD ,) -> NET_API_STATUS ; pub fn NetWkstaTransportEnum (servername : LPCWSTR , level : DWORD , bufptr : * mut LPBYTE , prefmaxlen : DWORD , entriesread : LPDWORD , totalentries : LPDWORD , resumehandle : LPDWORD ,) -> NET_API_STATUS ; }
};
}
