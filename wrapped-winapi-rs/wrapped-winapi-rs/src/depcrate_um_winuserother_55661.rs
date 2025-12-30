// Generated macro for other_55661 (other)
macro_rules! Depcrate_um_winuserother_55661 {
() => {
// Module: crate::um::winuser
// Provides: {"other_55661"}
// Dependencies: {}
extern "system" { pub fn GetDoubleClickTime () -> UINT ; pub fn SetDoubleClickTime (uInterval : UINT ,) -> BOOL ; pub fn RegisterClassA (lpWndClass : * const WNDCLASSA ,) -> ATOM ; pub fn RegisterClassW (lpWndClass : * const WNDCLASSW ,) -> ATOM ; pub fn UnregisterClassA (lpClassName : LPCSTR , hInstance : HINSTANCE ,) -> BOOL ; pub fn UnregisterClassW (lpClassName : LPCWSTR , hInstance : HINSTANCE ,) -> BOOL ; pub fn GetClassInfoA (hInstance : HINSTANCE , lpClassName : LPCSTR , lpWndClass : LPWNDCLASSA ,) -> BOOL ; pub fn GetClassInfoW (hInstance : HINSTANCE , lpClassName : LPCWSTR , lpWndClass : LPWNDCLASSW ,) -> BOOL ; pub fn RegisterClassExA (lpWndClass : * const WNDCLASSEXA ,) -> ATOM ; pub fn RegisterClassExW (lpWndClass : * const WNDCLASSEXW ,) -> ATOM ; pub fn GetClassInfoExA (hinst : HINSTANCE , lpszClass : LPCSTR , lpwcx : LPWNDCLASSEXA ,) -> BOOL ; pub fn GetClassInfoExW (hinst : HINSTANCE , lpszClass : LPCWSTR , lpwcx : LPWNDCLASSEXW ,) -> BOOL ; }
};
}
