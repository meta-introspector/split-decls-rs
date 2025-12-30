// Generated macro for other_30540 (other)
macro_rules! Depcrate_um_lmmsgother_30540 {
() => {
// Module: crate::um::lmmsg
// Provides: {"other_30540"}
// Dependencies: {}
extern "system" { pub fn NetMessageNameAdd (servername : LPCWSTR , msgname : LPCWSTR ,) -> NET_API_STATUS ; pub fn NetMessageNameEnum (servername : LPCWSTR , level : DWORD , bufptr : * mut LPBYTE , prefmaxlen : DWORD , entriesread : LPDWORD , totalentries : LPDWORD , resumehandle : LPDWORD ,) -> NET_API_STATUS ; pub fn NetMessageNameGetInfo (servername : LPCWSTR , msgname : LPCWSTR , level : DWORD , bufptr : * mut LPBYTE ,) -> NET_API_STATUS ; pub fn NetMessageNameDel (servername : LPCWSTR , msgname : LPCWSTR ,) -> NET_API_STATUS ; pub fn NetMessageBufferSend (servername : LPCWSTR , msgname : LPCWSTR , fromname : LPCWSTR , buf : LPBYTE , buflen : DWORD ,) -> NET_API_STATUS ; }
};
}
