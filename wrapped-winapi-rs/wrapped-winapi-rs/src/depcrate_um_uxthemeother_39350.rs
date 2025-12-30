// Generated macro for other_39350 (other)
macro_rules! Depcrate_um_uxthemeother_39350 {
() => {
// Module: crate::um::uxtheme
// Provides: {"other_39350"}
// Dependencies: {}
extern "system" { pub fn DrawThemeTextEx (hTheme : HTHEME , hdc : HDC , iPartId : c_int , iStateId : c_int , pszText : LPCWSTR , cchText : c_int , dwTextFlags : DWORD , pRect : LPRECT , pOptions : * const DTTOPTS ,) -> HRESULT ; pub fn GetThemeBitmap (hTheme : HTHEME , iPartId : c_int , iStateId : c_int , iPropId : c_int , dwFlags : ULONG , phBitmap : * mut HBITMAP ,) -> HRESULT ; pub fn GetThemeStream (hTheme : HTHEME , iPartId : c_int , iStateId : c_int , iPropId : c_int , ppvStream : * mut * mut VOID , pcbStream : * mut DWORD , hInst : HINSTANCE ,) -> HRESULT ; pub fn BufferedPaintInit () -> HRESULT ; pub fn BufferedPaintUnInit () -> HRESULT ; }
};
}
