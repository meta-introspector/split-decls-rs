// Generated macro for other_38345 (other)
macro_rules! Depcrate_um_shlobjother_38345 {
() => {
// Module: crate::um::shlobj
// Provides: {"other_38345"}
// Dependencies: {}
extern "system" { pub fn SHGetSpecialFolderLocation (hwnd : HWND , csidl : c_int , ppidl : * mut PIDLIST_ABSOLUTE ,) -> HRESULT ; pub fn SHCloneSpecialIDList (hwnd : HWND , csidl : c_int , fCreate : BOOL ,) -> PIDLIST_ABSOLUTE ; pub fn SHGetSpecialFolderPathA (hwnd : HWND , pszPath : LPSTR , csidl : c_int , fCreate : BOOL ,) -> BOOL ; pub fn SHGetSpecialFolderPathW (hwnd : HWND , pszPath : LPWSTR , csidl : c_int , fCreate : BOOL ,) -> BOOL ; pub fn SHFlushSFCache () ; }
};
}
