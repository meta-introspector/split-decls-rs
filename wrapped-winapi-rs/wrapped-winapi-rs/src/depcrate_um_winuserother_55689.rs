// Generated macro for other_55689 (other)
macro_rules! Depcrate_um_winuserother_55689 {
() => {
// Module: crate::um::winuser
// Provides: {"other_55689"}
// Dependencies: {}
extern "system" { pub fn ShowOwnedPopups (hWnd : HWND , fShow : BOOL ,) -> BOOL ; pub fn OpenIcon (hWnd : HWND ,) -> BOOL ; pub fn CloseWindow (hWnd : HWND ,) -> BOOL ; pub fn MoveWindow (hWnd : HWND , X : c_int , Y : c_int , nWidth : c_int , nHeight : c_int , bRepaint : BOOL ,) -> BOOL ; pub fn SetWindowPos (hWnd : HWND , hWndInsertAfter : HWND , X : c_int , Y : c_int , cx : c_int , cy : c_int , uFlags : UINT ,) -> BOOL ; pub fn GetWindowPlacement (hWnd : HWND , lpwndpl : * mut WINDOWPLACEMENT ,) -> BOOL ; pub fn SetWindowPlacement (hWnd : HWND , lpwndpl : * const WINDOWPLACEMENT ,) -> BOOL ; }
};
}
