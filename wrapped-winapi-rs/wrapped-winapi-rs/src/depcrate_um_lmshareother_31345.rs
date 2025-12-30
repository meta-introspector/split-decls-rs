// Generated macro for other_31345 (other)
macro_rules! Depcrate_um_lmshareother_31345 {
() => {
// Module: crate::um::lmshare
// Provides: {"other_31345"}
// Dependencies: {}
extern "system" { pub fn NetSessionEnum (servername : LMSTR , UncClientName : LMSTR , username : LMSTR , level : DWORD , bufptr : * mut LPBYTE , prefmaxlen : DWORD , entriesread : LPDWORD , totalentries : LPDWORD , resumehandle : LPDWORD ,) -> NET_API_STATUS ; pub fn NetSessionDel (servername : LMSTR , UncClientName : LMSTR , username : LMSTR ,) -> NET_API_STATUS ; pub fn NetSessionGetInfo (servername : LMSTR , UncClientName : LMSTR , username : LMSTR , level : DWORD , bufptr : * mut LPBYTE ,) -> NET_API_STATUS ; }
};
}
