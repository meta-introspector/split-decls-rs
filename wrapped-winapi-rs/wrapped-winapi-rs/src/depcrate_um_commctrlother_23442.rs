// Generated macro for other_23442 (other)
macro_rules! Depcrate_um_commctrlother_23442 {
() => {
// Module: crate::um::commctrl
// Provides: {"other_23442"}
// Dependencies: {}
extern "system" { pub fn FlatSB_EnableScrollBar (hWnd : HWND , wSBflags : c_int , wArrows : UINT ,) -> BOOL ; pub fn FlatSB_ShowScrollBar (hWnd : HWND , code : c_int , fShow : BOOL ,) -> BOOL ; pub fn FlatSB_GetScrollRange (hWnd : HWND , code : c_int , lpMinPos : LPINT , lpMaxPos : LPINT ,) -> BOOL ; pub fn FlatSB_GetScrollInfo (hwnd : HWND , code : c_int , lpsi : LPSCROLLINFO ,) -> BOOL ; pub fn FlatSB_GetScrollPos (hWnd : HWND , code : c_int ,) -> c_int ; pub fn FlatSB_GetScrollProp (hWnd : HWND , propIndex : c_int , pValue : LPINT ,) -> BOOL ; # [cfg (target_pointer_width = "64")] pub fn FlatSB_GetScrollPropPtr (hWnd : HWND , propIndex : c_int , pValue : PINT_PTR ,) -> BOOL ; pub fn FlatSB_SetScrollPos (hWnd : HWND , code : c_int , pos : c_int , fRedraw : BOOL ,) -> c_int ; pub fn FlatSB_SetScrollInfo (hWnd : HWND , code : c_int , psi : LPSCROLLINFO , fRedraw : BOOL ,) -> c_int ; pub fn FlatSB_SetScrollRange (hWnd : HWND , code : c_int , min : c_int , max : c_int , fRedraw : BOOL ,) -> c_int ; pub fn FlatSB_SetScrollProp (hWnd : HWND , index : UINT , newValue : INT_PTR , fRedraw : BOOL ,) -> BOOL ; pub fn InitializeFlatSB (hWnd : HWND ,) -> BOOL ; pub fn UninitializeFlatSB (hWnd : HWND ,) -> HRESULT ; }
};
}
