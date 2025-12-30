// Generated macro for other_55990 (other)
macro_rules! Depcrate_um_winuserother_55990 {
() => {
// Module: crate::um::winuser
// Provides: {"other_55990"}
// Dependencies: {}
extern "system" { pub fn TrackPopupMenuEx (hMenu : HMENU , uFlags : UINT , x : INT , y : INT , hwnd : HWND , lptpm : LPTPMPARAMS ,) -> BOOL ; pub fn CalculatePopupWindowPosition (anchorPoint : * const POINT , windowSize : * const SIZE , flags : UINT , excludeRect : * mut RECT , popupWindowPosition : * mut RECT ,) -> BOOL ; }
};
}
