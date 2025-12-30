// Generated macro for other_39262 (other)
macro_rules! Depcrate_um_uxthemeother_39262 {
() => {
// Module: crate::um::uxtheme
// Provides: {"other_39262"}
// Dependencies: {}
extern "system" { pub fn OpenThemeDataForDpi (hwnd : HWND , pszClassList : LPCWSTR , dpi : UINT ,) -> HTHEME ; pub fn OpenThemeDataEx (hwnd : HWND , pszClassList : LPCWSTR , dwFlags : DWORD ,) -> HTHEME ; pub fn CloseThemeData (hTheme : HTHEME ,) -> HRESULT ; pub fn DrawThemeBackground (hTheme : HTHEME , hdc : HDC , iPartId : c_int , iStateId : c_int , pRect : LPCRECT , pClipRect : LPCRECT ,) -> HRESULT ; }
};
}
