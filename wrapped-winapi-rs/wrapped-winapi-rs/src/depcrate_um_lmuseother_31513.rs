// Generated macro for other_31513 (other)
macro_rules! Depcrate_um_lmuseother_31513 {
() => {
// Module: crate::um::lmuse
// Provides: {"other_31513"}
// Dependencies: {}
extern "system" { pub fn NetUseAdd (servername : LPWSTR , level : DWORD , buf : LPBYTE , parm_err : LPDWORD ,) -> NET_API_STATUS ; pub fn NetUseDel (UncServerName : LMSTR , UseName : LMSTR , ForceCond : DWORD ,) -> NET_API_STATUS ; pub fn NetUseEnum (UncServerName : LMSTR , Level : DWORD , BufPtr : * mut LPBYTE , PreferedMaximumSize : DWORD , EntriesRead : LPDWORD , TotalEntries : LPDWORD , ResumeHandle : LPDWORD ,) -> NET_API_STATUS ; pub fn NetUseGetInfo (UncServerName : LMSTR , UseName : LMSTR , level : DWORD , bufptr : * mut LPBYTE ,) -> NET_API_STATUS ; }
};
}
