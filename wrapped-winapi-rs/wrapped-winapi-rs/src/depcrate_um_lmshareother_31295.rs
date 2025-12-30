// Generated macro for other_31295 (other)
macro_rules! Depcrate_um_lmshareother_31295 {
() => {
// Module: crate::um::lmshare
// Provides: {"other_31295"}
// Dependencies: {}
extern "system" { pub fn NetServerAliasAdd (servername : LMSTR , level : DWORD , buf : LPBYTE ,) -> NET_API_STATUS ; pub fn NetServerAliasDel (servername : LMSTR , level : DWORD , buf : LPBYTE ,) -> NET_API_STATUS ; pub fn NetServerAliasEnum (servername : LMSTR , level : DWORD , bufptr : * mut LPBYTE , prefmaxlen : DWORD , entriesread : LPDWORD , totalentries : LPDWORD , resumehandle : LPDWORD ,) -> NET_API_STATUS ; }
};
}
