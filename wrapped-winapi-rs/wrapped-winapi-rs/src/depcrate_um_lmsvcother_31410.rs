// Generated macro for other_31410 (other)
macro_rules! Depcrate_um_lmsvcother_31410 {
() => {
// Module: crate::um::lmsvc
// Provides: {"other_31410"}
// Dependencies: {}
extern "system" { pub fn NetServiceControl (servername : LPCWSTR , service : LPCWSTR , opcode : DWORD , arg : DWORD , bufptr : * mut LPBYTE ,) -> NET_API_STATUS ; pub fn NetServiceEnum (servername : LPCWSTR , level : DWORD , bufptr : * mut LPBYTE , prefmaxlen : DWORD , entriesread : LPDWORD , totalentries : LPDWORD , resumehandle : LPDWORD ,) -> NET_API_STATUS ; pub fn NetServiceGetInfo (servername : LPCWSTR , service : LPCWSTR , level : DWORD , bufptr : * mut LPBYTE ,) -> NET_API_STATUS ; pub fn NetServiceInstall (servername : LPCWSTR , service : LPCWSTR , argc : DWORD , argv : * mut LPCWSTR , bufptr : * mut LPBYTE ,) -> NET_API_STATUS ; }
};
}
