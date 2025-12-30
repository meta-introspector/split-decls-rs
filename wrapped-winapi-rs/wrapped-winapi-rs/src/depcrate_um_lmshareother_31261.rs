// Generated macro for other_31261 (other)
macro_rules! Depcrate_um_lmshareother_31261 {
() => {
// Module: crate::um::lmshare
// Provides: {"other_31261"}
// Dependencies: {}
extern "system" { pub fn NetShareAdd (servername : LMSTR , level : DWORD , buf : LPBYTE , parm_err : LPDWORD ,) -> NET_API_STATUS ; pub fn NetShareEnum (servername : LMSTR , level : DWORD , bufptr : * mut LPBYTE , prefmaxlen : DWORD , entriesread : LPDWORD , totalentries : LPDWORD , resumehandle : LPDWORD ,) -> NET_API_STATUS ; pub fn NetShareEnumSticky (servername : LMSTR , level : DWORD , bufptr : * mut LPBYTE , prefmaxlen : DWORD , entriesread : LPDWORD , totalentries : LPDWORD , resumehandle : LPDWORD ,) -> NET_API_STATUS ; pub fn NetShareGetInfo (servername : LMSTR , netname : LMSTR , level : DWORD , bufptr : * mut LPBYTE ,) -> NET_API_STATUS ; pub fn NetShareSetInfo (servername : LMSTR , netname : LMSTR , level : DWORD , buf : LPBYTE , parm_err : LPDWORD ,) -> NET_API_STATUS ; pub fn NetShareDel (servername : LMSTR , netname : LMSTR , reserved : DWORD ,) -> NET_API_STATUS ; pub fn NetShareDelSticky (servername : LMSTR , netname : LMSTR , reserved : DWORD ,) -> NET_API_STATUS ; pub fn NetShareCheck (servername : LMSTR , device : LMSTR , _type : LPDWORD ,) -> NET_API_STATUS ; pub fn NetShareDelEx (servername : LMSTR , level : DWORD , buf : LPBYTE ,) -> NET_API_STATUS ; }
};
}
