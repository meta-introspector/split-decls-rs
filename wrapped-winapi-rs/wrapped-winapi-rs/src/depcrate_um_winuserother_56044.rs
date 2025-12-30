// Generated macro for other_56044 (other)
macro_rules! Depcrate_um_winuserother_56044 {
() => {
// Module: crate::um::winuser
// Provides: {"other_56044"}
// Dependencies: {}
extern "system" { pub fn GetMenuDefaultItem (hMenu : HMENU , fByPos : UINT , gmdiFlags : UINT ,) -> UINT ; pub fn SetMenuDefaultItem (hMenu : HMENU , uItem : UINT , fByPos : UINT ,) -> BOOL ; pub fn GetMenuItemRect (hWnd : HWND , hMenu : HMENU , uItem : UINT , lprcItem : LPRECT ,) -> BOOL ; pub fn MenuItemFromPoint (hWnd : HWND , hMenu : HMENU , ptScreen : POINT ,) -> c_int ; }
};
}
