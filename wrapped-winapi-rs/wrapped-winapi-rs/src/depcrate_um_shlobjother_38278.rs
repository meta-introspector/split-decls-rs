// Generated macro for other_38278 (other)
macro_rules! Depcrate_um_shlobjother_38278 {
() => {
// Module: crate::um::shlobj
// Provides: {"other_38278"}
// Dependencies: {}
extern "system" { pub fn SHGetPathFromIDListEx (pidl : PCIDLIST_ABSOLUTE , pszPath : PWSTR , cchPath : DWORD , uOpts : GPFIDL_FLAGS ,) -> BOOL ; pub fn SHGetPathFromIDListA (pidl : PCIDLIST_ABSOLUTE , pszPath : LPSTR ,) -> BOOL ; pub fn SHGetPathFromIDListW (pidl : PCIDLIST_ABSOLUTE , pszPath : LPWSTR ,) -> BOOL ; pub fn SHCreateDirectory (hwnd : HWND , pszPath : PCWSTR ,) -> c_int ; pub fn SHCreateDirectoryExA (hwnd : HWND , pszPath : LPCSTR , psa : * const SECURITY_ATTRIBUTES ,) -> c_int ; pub fn SHCreateDirectoryExW (hwnd : HWND , pszPath : LPCWSTR , psa : * const SECURITY_ATTRIBUTES ,) -> c_int ; }
};
}
