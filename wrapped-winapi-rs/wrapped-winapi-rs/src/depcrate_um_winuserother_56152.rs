// Generated macro for other_56152 (other)
macro_rules! Depcrate_um_winuserother_56152 {
() => {
// Module: crate::um::winuser
// Provides: {"other_56152"}
// Dependencies: {}
extern "system" { pub fn SetScrollPos (hWnd : HWND , nBar : c_int , nPos : c_int , bRedraw : BOOL ,) -> c_int ; pub fn GetScrollPos (hWnd : HWND , nBar : c_int ,) -> c_int ; pub fn SetScrollRange (hWnd : HWND , nBar : c_int , nMinPos : c_int , nMaxPos : c_int , bRedraw : BOOL ,) -> BOOL ; pub fn GetScrollRange (hWnd : HWND , nBar : c_int , lpMinPos : LPINT , lpMaxPos : LPINT ,) -> BOOL ; pub fn ShowScrollBar (hWnd : HWND , wBar : c_int , bShow : BOOL ,) -> BOOL ; pub fn EnableScrollBar (hWnd : HWND , wSBflags : UINT , wArrows : UINT ,) -> BOOL ; }
};
}
