// Generated macro for other_38240 (other)
macro_rules! Depcrate_um_shellapiother_38240 {
() => {
// Module: crate::um::shellapi
// Provides: {"other_38240"}
// Dependencies: {}
extern "system" { pub fn IsLFNDriveA (pszPath : LPCSTR ,) -> BOOL ; pub fn IsLFNDriveW (pszPath : LPCWSTR ,) -> BOOL ; pub fn SHEnumerateUnreadMailAccountsA (hKeyUser : HKEY , dwIndex : DWORD , pszMailAddress : LPSTR , cchMailAddress : c_int ,) -> HRESULT ; pub fn SHEnumerateUnreadMailAccountsW (hKeyUser : HKEY , dwIndex : DWORD , pszMailAddress : LPWSTR , cchMailAddress : c_int ,) -> HRESULT ; pub fn SHGetUnreadMailCountA (hKeyUser : HKEY , pszMailAddress : LPCSTR , pdwCount : * mut DWORD , pFileTime : * mut FILETIME , pszShellExecuteCommand : LPSTR , cchShellExecuteCommand : c_int ,) -> HRESULT ; pub fn SHGetUnreadMailCountW (hKeyUser : HKEY , pszMailAddress : LPCWSTR , pdwCount : * mut DWORD , pFileTime : * mut FILETIME , pszShellExecuteCommand : LPWSTR , cchShellExecuteCommand : c_int ,) -> HRESULT ; pub fn SHSetUnreadMailCountA (pszMailAddress : LPCSTR , dwCount : DWORD , pszShellExecuteCommand : LPCSTR ,) -> HRESULT ; pub fn SHSetUnreadMailCountW (pszMailAddress : LPCWSTR , dwCount : DWORD , pszShellExecuteCommand : LPCWSTR ,) -> HRESULT ; pub fn SHTestTokenMembership (hToken : HANDLE , ulRID : ULONG ,) -> BOOL ; pub fn SHGetImageList (iImageList : c_int , riid : REFIID , ppvObj : * mut * mut c_void ,) -> HRESULT ; }
};
}
