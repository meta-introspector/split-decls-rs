// Generated macro for other_55692 (other)
macro_rules! Depcrate_um_winuserother_55692 {
() => {
// Module: crate::um::winuser
// Provides: {"other_55692"}
// Dependencies: {}
extern "system" { pub fn GetWindowDisplayAffinity (hWnd : HWND , pdwAffinity : * mut DWORD ,) -> BOOL ; pub fn SetWindowDisplayAffinity (hWnd : HWND , dwAffinity : DWORD ,) -> BOOL ; pub fn BeginDeferWindowPos (nNumWindows : c_int ,) -> HDWP ; pub fn DeferWindowPos (hWinPosInfo : HDWP , hWnd : HWND , hWndInserAfter : HWND , x : c_int , y : c_int , cx : c_int , cy : c_int , uFlags : UINT ,) -> HDWP ; pub fn EndDeferWindowPos (hWinPosInfo : HDWP ,) -> BOOL ; pub fn IsWindowVisible (hWnd : HWND ,) -> BOOL ; pub fn IsIconic (hWnd : HWND ,) -> BOOL ; pub fn AnyPopup () -> BOOL ; pub fn BringWindowToTop (hWnd : HWND ,) -> BOOL ; pub fn IsZoomed (hwnd : HWND ,) -> BOOL ; }
};
}
