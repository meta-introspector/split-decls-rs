// Generated macro for other_38213 (other)
macro_rules! Depcrate_um_shellapiother_38213 {
() => {
// Module: crate::um::shellapi
// Provides: {"other_38213"}
// Dependencies: {}
extern "system" { pub fn SHGetStockIconInfo (siid : SHSTOCKICONID , uFlags : UINT , psii : * mut SHSTOCKICONINFO ,) -> HRESULT ; pub fn SHGetDiskFreeSpaceExA (pszDirectoryName : LPCSTR , pulFreeBytesAvailableToCaller : * mut ULARGE_INTEGER , pulTotalNumberOfBytes : * mut ULARGE_INTEGER , pulTotalNumberOfFreeBytes : * mut ULARGE_INTEGER ,) -> BOOL ; pub fn SHGetDiskFreeSpaceExW (pszDirectoryName : LPCWSTR , pulFreeBytesAvailableToCaller : * mut ULARGE_INTEGER , pulTotalNumberOfBytes : * mut ULARGE_INTEGER , pulTotalNumberOfFreeBytes : * mut ULARGE_INTEGER ,) -> BOOL ; pub fn SHGetNewLinkInfoA (pszLinkTo : LPCSTR , pszDir : LPCSTR , pszName : LPSTR , pfMustCopy : * mut BOOL , uFlags : UINT ,) -> BOOL ; pub fn SHGetNewLinkInfoW (pszLinkTo : LPCWSTR , pszDir : LPCWSTR , pszName : LPWSTR , pfMustCopy : * mut BOOL , uFlags : UINT ,) -> BOOL ; }
};
}
