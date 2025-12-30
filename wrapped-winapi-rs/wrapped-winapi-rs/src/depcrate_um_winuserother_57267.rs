// Generated macro for other_57267 (other)
macro_rules! Depcrate_um_winuserother_57267 {
() => {
// Module: crate::um::winuser
// Provides: {"other_57267"}
// Dependencies: {}
extern "system" { pub fn GetAncestor (hwnd : HWND , gaFlags : UINT ,) -> HWND ; pub fn RealChildWindowFromPoint (hwndParent : HWND , ptParentClientCoords : POINT ,) -> HWND ; pub fn RealGetWindowClassA (hwnd : HWND , ptszClassName : LPSTR , cchClassNameMax : UINT ,) -> UINT ; pub fn RealGetWindowClassW (hwnd : HWND , ptszClassName : LPWSTR , cchClassNameMax : UINT ,) -> UINT ; }
};
}
