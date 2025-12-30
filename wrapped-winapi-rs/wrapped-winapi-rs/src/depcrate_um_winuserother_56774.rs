// Generated macro for other_56774 (other)
macro_rules! Depcrate_um_winuserother_56774 {
() => {
// Module: crate::um::winuser
// Provides: {"other_56774"}
// Dependencies: {}
extern "system" { pub fn SetScrollInfo (hwnd : HWND , nBar : c_int , lpsi : * const SCROLLINFO , redraw : BOOL ,) -> c_int ; pub fn GetScrollInfo (hwnd : HWND , nBar : c_int , lpsi : * mut SCROLLINFO ,) -> BOOL ; }
};
}
