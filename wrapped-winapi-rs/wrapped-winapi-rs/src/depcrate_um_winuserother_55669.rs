// Generated macro for other_55669 (other)
macro_rules! Depcrate_um_winuserother_55669 {
() => {
// Module: crate::um::winuser
// Provides: {"other_55669"}
// Dependencies: {}
extern "system" { pub fn UpdateLayeredWindowIndirect (hWnd : HWND , pULWInfo : * mut UPDATELAYEREDWINDOWINFO ,) -> BOOL ; pub fn GetLayeredWindowAttributes (hwnd : HWND , pcrKey : * mut COLORREF , pbAlpha : * mut BYTE , pdwFlags : * mut DWORD ,) -> BOOL ; }
};
}
