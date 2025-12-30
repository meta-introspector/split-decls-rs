// Generated macro for other_55666 (other)
macro_rules! Depcrate_um_winuserother_55666 {
() => {
// Module: crate::um::winuser
// Provides: {"other_55666"}
// Dependencies: {}
extern "system" { pub fn IsWindow (hWnd : HWND ,) -> BOOL ; pub fn IsMenu (hMenu : HMENU ,) -> BOOL ; pub fn IsChild (hWndParent : HWND , hWnd : HWND ,) -> BOOL ; pub fn DestroyWindow (hWnd : HWND ,) -> BOOL ; pub fn ShowWindow (hWnd : HWND , nCmdShow : c_int ,) -> BOOL ; pub fn AnimateWindow (hWnd : HWND , dwTime : DWORD , dwFlags : DWORD ,) -> BOOL ; pub fn UpdateLayeredWindow (hWnd : HWND , hdcDst : HDC , pptDst : * mut POINT , psize : * mut SIZE , hdcSrc : HDC , pptSrc : * mut POINT , crKey : COLORREF , pblend : * mut BLENDFUNCTION , dwFlags : DWORD ,) -> BOOL ; }
};
}
