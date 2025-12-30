// Generated macro for other_29843 (other)
macro_rules! Depcrate_um_lmaccessother_29843 {
() => {
// Module: crate::um::lmaccess
// Provides: {"other_29843"}
// Dependencies: {}
extern "system" { pub fn NetGroupAdd (servername : LPCWSTR , level : DWORD , buf : LPBYTE , parm_err : LPDWORD ,) -> NET_API_STATUS ; pub fn NetGroupAddUser (servername : LPCWSTR , GroupName : LPCWSTR , username : LPCWSTR ,) -> NET_API_STATUS ; pub fn NetGroupEnum (servername : LPCWSTR , level : DWORD , bufptr : * mut LPBYTE , prefmaxlen : DWORD , entriesread : LPDWORD , totalentries : LPDWORD , resume_handle : PDWORD_PTR ,) -> NET_API_STATUS ; pub fn NetGroupGetInfo (servername : LPCWSTR , groupname : LPCWSTR , level : DWORD , bufptr : * mut LPBYTE ,) -> NET_API_STATUS ; pub fn NetGroupSetInfo (servername : LPCWSTR , groupname : LPCWSTR , level : DWORD , buf : LPBYTE , parm_err : LPDWORD ,) -> NET_API_STATUS ; pub fn NetGroupDel (servername : LPCWSTR , groupname : LPCWSTR ,) -> NET_API_STATUS ; pub fn NetGroupDelUser (servername : LPCWSTR , GroupName : LPCWSTR , Username : LPCWSTR ,) -> NET_API_STATUS ; pub fn NetGroupGetUsers (servername : LPCWSTR , groupname : LPCWSTR , level : DWORD , bufptr : * mut LPBYTE , prefmaxlen : DWORD , entriesread : LPDWORD , totalentries : LPDWORD , ResumeHandle : PDWORD_PTR ,) -> NET_API_STATUS ; pub fn NetGroupSetUsers (servername : LPCWSTR , groupname : LPCWSTR , level : DWORD , buf : LPBYTE , totalentries : DWORD ,) -> NET_API_STATUS ; }
};
}
