// Generated macro for other_31372 (other)
macro_rules! Depcrate_um_lmshareother_31372 {
() => {
// Module: crate::um::lmshare
// Provides: {"other_31372"}
// Dependencies: {}
extern "system" { pub fn NetFileClose (servername : LMSTR , fileid : DWORD ,) -> NET_API_STATUS ; pub fn NetFileEnum (servername : LMSTR , basepath : LMSTR , username : LMSTR , level : DWORD , bufptr : * mut LPBYTE , prefmaxlen : DWORD , entriesread : LPDWORD , totalentries : LPDWORD , resume_handle : PDWORD_PTR ,) -> NET_API_STATUS ; pub fn NetFileGetInfo (servername : LMSTR , fileid : DWORD , level : DWORD , bufptr : * mut LPBYTE ,) -> NET_API_STATUS ; }
};
}
