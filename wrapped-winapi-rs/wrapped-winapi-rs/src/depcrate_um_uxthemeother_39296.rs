// Generated macro for other_39296 (other)
macro_rules! Depcrate_um_uxthemeother_39296 {
() => {
// Module: crate::um::uxtheme
// Provides: {"other_39296"}
// Dependencies: {}
extern "system" { pub fn GetThemePropertyOrigin (hTheme : HTHEME , iPartId : c_int , iStateId : c_int , iPropId : c_int , pOrigin : * mut PROPERTYORIGIN ,) -> HRESULT ; pub fn SetWindowTheme (hwnd : HWND , pszSubAppName : LPCWSTR , pszSubIdList : LPCWSTR ,) -> HRESULT ; pub fn GetThemeFilename (hTheme : HTHEME , iPartId : c_int , iStateId : c_int , iPropId : c_int , pszThemeFileName : LPWSTR , cchMaxBuffChars : c_int ,) -> HRESULT ; pub fn GetThemeSysColor (hTheme : HTHEME , iColorId : c_int ,) -> COLORREF ; pub fn GetThemeSysColorBrush (hTheme : HTHEME , iColorId : c_int ,) -> HBRUSH ; pub fn GetThemeSysBool (hTheme : HTHEME , iBoolId : c_int ,) -> BOOL ; pub fn GetThemeSysSize (hTheme : HTHEME , iSizeId : c_int ,) -> c_int ; pub fn GetThemeSysFont (hTheme : HTHEME , iFontId : c_int , plf : * mut LOGFONTW ,) -> HRESULT ; pub fn GetThemeSysString (hTheme : HTHEME , iStringId : c_int , pszStringBuff : LPWSTR , cchMaxStringChars : c_int ,) -> HRESULT ; pub fn GetThemeSysInt (hTheme : HTHEME , iIntId : c_int , piValue : * mut c_int ,) -> HRESULT ; pub fn IsThemeActive () -> BOOL ; pub fn IsAppThemed () -> BOOL ; pub fn GetWindowTheme (hwnd : HWND ,) -> HTHEME ; }
};
}
