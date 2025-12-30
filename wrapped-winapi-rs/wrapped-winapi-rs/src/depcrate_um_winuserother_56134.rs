// Generated macro for other_56134 (other)
macro_rules! Depcrate_um_winuserother_56134 {
() => {
// Module: crate::um::winuser
// Provides: {"other_56134"}
// Dependencies: {}
extern "system" { pub fn GetWindowDC (hWnd : HWND ,) -> HDC ; pub fn ReleaseDC (hWnd : HWND , hDC : HDC ,) -> c_int ; pub fn BeginPaint (hwnd : HWND , lpPaint : LPPAINTSTRUCT ,) -> HDC ; pub fn EndPaint (hWnd : HWND , lpPaint : * const PAINTSTRUCT ,) -> BOOL ; pub fn GetUpdateRect (hWnd : HWND , lpRect : LPRECT , bErase : BOOL ,) -> BOOL ; pub fn GetUpdateRgn (hWnd : HWND , hRgn : HRGN , bErase : BOOL ,) -> c_int ; pub fn SetWindowRgn (hWnd : HWND , hRgn : HRGN , bRedraw : BOOL ,) -> c_int ; pub fn GetWindowRgn (hWnd : HWND , hRgn : HRGN ,) -> c_int ; pub fn GetWindowRgnBox (hWnd : HWND , lprc : LPRECT ,) -> c_int ; pub fn ExcludeUpdateRgn (hDC : HDC , hWnd : HWND ,) -> c_int ; pub fn InvalidateRect (hWnd : HWND , lpRect : * const RECT , bErase : BOOL ,) -> BOOL ; pub fn ValidateRect (hWnd : HWND , lpRect : * const RECT ,) -> BOOL ; pub fn InvalidateRgn (hWnd : HWND , hRgn : HRGN , bErase : BOOL ,) -> BOOL ; pub fn ValidateRgn (hWnd : HWND , hRgn : HRGN ,) -> BOOL ; pub fn RedrawWindow (hwnd : HWND , lprcUpdate : * const RECT , hrgnUpdate : HRGN , flags : UINT ,) -> BOOL ; }
};
}
