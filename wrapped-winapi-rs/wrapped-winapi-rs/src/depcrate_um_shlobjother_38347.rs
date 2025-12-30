// Generated macro for other_38347 (other)
macro_rules! Depcrate_um_shlobjother_38347 {
() => {
// Module: crate::um::shlobj
// Provides: {"other_38347"}
// Dependencies: {}
extern "system" { pub fn SHGetFolderPathA (hwnd : HWND , csidl : c_int , hToken : HANDLE , dwFlags : DWORD , pszPath : LPSTR ,) -> HRESULT ; pub fn SHGetFolderPathW (hwnd : HWND , csidl : c_int , hToken : HANDLE , dwFlags : DWORD , pszPath : LPWSTR ,) -> HRESULT ; pub fn SHGetFolderLocation (hwnd : HWND , csidl : c_int , hToken : HANDLE , dwFlags : DWORD , ppidl : * mut PIDLIST_ABSOLUTE ,) -> HRESULT ; pub fn SHSetFolderPathA (csidl : c_int , hToken : HANDLE , dwFlags : DWORD , pszPath : LPCSTR ,) -> HRESULT ; pub fn SHSetFolderPathW (csidl : c_int , hToken : HANDLE , dwFlags : DWORD , pszPath : LPCWSTR ,) -> HRESULT ; pub fn SHGetFolderPathAndSubDirA (hwnd : HWND , csidl : c_int , hToken : HANDLE , dwFlags : DWORD , pszSubDir : LPCSTR , pszPath : LPSTR ,) -> HRESULT ; pub fn SHGetFolderPathAndSubDirW (hwnd : HWND , csidl : c_int , hToken : HANDLE , dwFlags : DWORD , pszSubDir : LPCWSTR , pszPath : LPWSTR ,) -> HRESULT ; }
};
}
