// Generated macro for other_38048 (other)
macro_rules! Depcrate_um_shellapiother_38048 {
() => {
// Module: crate::um::shellapi
// Provides: {"other_38048"}
// Dependencies: {}
extern "system" { pub fn SHAppBarMessage (dwMessage : DWORD , pData : PAPPBARDATA ,) -> UINT_PTR ; pub fn DoEnvironmentSubstA (pszSrc : LPSTR , cchSrc : UINT ,) -> DWORD ; pub fn DoEnvironmentSubstW (pszSrc : LPWSTR , cchSrc : UINT ,) -> DWORD ; pub fn ExtractIconExA (lpszFile : LPCSTR , nIconIndex : c_int , phiconLarge : * mut HICON , phiconSmall : * mut HICON , nIcons : UINT ,) -> UINT ; pub fn ExtractIconExW (lpszFile : LPCWSTR , nIconIndex : c_int , phiconLarge : * mut HICON , phiconSmall : * mut HICON , nIcons : UINT ,) -> UINT ; }
};
}
