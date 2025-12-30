// Generated macro for other_29913 (other)
macro_rules! Depcrate_um_lmaccessother_29913 {
() => {
// Module: crate::um::lmaccess
// Provides: {"other_29913"}
// Dependencies: {}
extern "system" { pub fn NetAccessAdd (servername : LPCWSTR , level : DWORD , buf : LPBYTE , parm_err : LPDWORD ,) -> NET_API_STATUS ; pub fn NetAccessEnum (servername : LPCWSTR , BasePath : LPCWSTR , Recursive : DWORD , level : DWORD , bufptr : * mut LPBYTE , prefmaxlen : DWORD , entriesread : LPDWORD , totalentries : LPDWORD , resume_handle : LPDWORD ,) -> NET_API_STATUS ; pub fn NetAccessGetInfo (servername : LPCWSTR , resource : LPCWSTR , level : DWORD , bufptr : * mut LPBYTE ,) -> NET_API_STATUS ; pub fn NetAccessSetInfo (servername : LPCWSTR , resource : LPCWSTR , level : DWORD , buf : LPBYTE , parm_err : LPDWORD ,) -> NET_API_STATUS ; pub fn NetAccessDel (servername : LPCWSTR , resource : LPCWSTR ,) -> NET_API_STATUS ; pub fn NetAccessGetUserPerms (servername : LPCWSTR , UGname : LPCWSTR , resource : LPCWSTR , Perms : LPDWORD ,) -> NET_API_STATUS ; }
};
}
