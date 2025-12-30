// Generated macro for other_38201 (other)
macro_rules! Depcrate_um_shellapiother_38201 {
() => {
// Module: crate::um::shellapi
// Provides: {"other_38201"}
// Dependencies: {}
extern "system" { pub fn SHGetFileInfoA (pszPath : LPCSTR , dwFileAttributes : DWORD , psfi : * mut SHFILEINFOA , cbFileInfo : UINT , uFlags : UINT ,) -> DWORD_PTR ; pub fn SHGetFileInfoW (pszPath : LPCWSTR , dwFileAttributes : DWORD , psfi : * mut SHFILEINFOW , cbFileInfo : UINT , uFlags : UINT ,) -> DWORD_PTR ; }
};
}
