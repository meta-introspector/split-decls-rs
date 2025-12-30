// Generated macro for other_30596 (other)
macro_rules! Depcrate_um_lmreplother_30596 {
() => {
// Module: crate::um::lmrepl
// Provides: {"other_30596"}
// Dependencies: {}
extern "system" { pub fn NetReplGetInfo (servername : LPCWSTR , level : DWORD , bufptr : * mut LPBYTE ,) -> NET_API_STATUS ; pub fn NetReplSetInfo (servername : LPCWSTR , level : DWORD , buf : LPBYTE , parm_err : LPDWORD ,) -> NET_API_STATUS ; }
};
}
