// Generated macro for other_38134 (other)
macro_rules! Depcrate_um_shellapiother_38134 {
() => {
// Module: crate::um::shellapi
// Provides: {"other_38134"}
// Dependencies: {}
extern "system" { pub fn SHQueryRecycleBinA (pszRootPath : LPCSTR , pSHQueryRBInfo : LPSHQUERYRBINFO ,) -> HRESULT ; pub fn SHQueryRecycleBinW (pszRootPath : LPCWSTR , pSHQueryRBInfo : LPSHQUERYRBINFO ,) -> HRESULT ; pub fn SHEmptyRecycleBinA (hwnd : HWND , pszRootPath : LPCSTR , dwFlags : DWORD ,) -> HRESULT ; pub fn SHEmptyRecycleBinW (hwnd : HWND , pszRootPath : LPCWSTR , dwFlags : DWORD ,) -> HRESULT ; }
};
}
