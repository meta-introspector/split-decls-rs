// Generated macro for other_39276 (other)
macro_rules! Depcrate_um_uxthemeother_39276 {
() => {
// Module: crate::um::uxtheme
// Provides: {"other_39276"}
// Dependencies: {}
extern "system" { pub fn GetThemePartSize (hTheme : HTHEME , hdc : HDC , iPartId : c_int , iStateId : c_int , prc : LPCRECT , eSize : THEMESIZE , psz : * mut SIZE ,) -> HRESULT ; pub fn GetThemeTextExtent (hTheme : HTHEME , hdc : HDC , iPartId : c_int , iStateId : c_int , pszText : LPCWSTR , cchCharCount : c_int , dwTextFlags : DWORD , pBoundingRect : LPCRECT , pExtentRect : LPRECT ,) -> HRESULT ; pub fn GetThemeTextMetrics (hTheme : HTHEME , hdc : HDC , iPartId : c_int , iStateId : c_int , ptm : * mut TEXTMETRICW ,) -> HRESULT ; }
};
}
