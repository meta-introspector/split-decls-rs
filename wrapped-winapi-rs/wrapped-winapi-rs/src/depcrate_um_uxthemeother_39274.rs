// Generated macro for other_39274 (other)
macro_rules! Depcrate_um_uxthemeother_39274 {
() => {
// Module: crate::um::uxtheme
// Provides: {"other_39274"}
// Dependencies: {}
extern "system" { pub fn DrawThemeText (hTheme : HTHEME , hdc : HDC , iPartId : c_int , iStateId : c_int , pszText : LPCWSTR , cchText : c_int , dwTextFlags : DWORD , dwTextFlags2 : DWORD , pRect : LPCRECT ,) -> HRESULT ; pub fn GetThemeBackgroundContentRect (hTheme : HTHEME , hdc : HDC , iPartId : c_int , iStateId : c_int , pBoundingRect : LPCRECT , pContentRect : LPRECT ,) -> HRESULT ; pub fn GetThemeBackgroundExtent (hTheme : HTHEME , hdc : HDC , iPartId : c_int , iStateId : c_int , pContentRect : LPCRECT , pExtentRect : LPRECT ,) -> HRESULT ; pub fn GetThemeBackgroundRegion (hTheme : HTHEME , hdc : HDC , iPartId : c_int , iStateId : c_int , pRect : LPCRECT , pRegion : * mut HRGN ,) -> HRESULT ; }
};
}
