// Generated macro for other_57041 (other)
macro_rules! Depcrate_um_winuserother_57041 {
() => {
// Module: crate::um::winuser
// Provides: {"other_57041"}
// Dependencies: {}
extern "system" { pub fn ChangeDisplaySettingsA (lpDevMode : * mut DEVMODEA , dwFlags : DWORD ,) -> LONG ; pub fn ChangeDisplaySettingsW (lpDevMode : * mut DEVMODEW , dwFlags : DWORD ,) -> LONG ; pub fn ChangeDisplaySettingsExA (lpszDeviceName : LPCSTR , lpDevMode : * mut DEVMODEA , hwnd : HWND , dwFlags : DWORD , lParam : LPVOID ,) -> LONG ; pub fn ChangeDisplaySettingsExW (lpszDeviceName : LPCWSTR , lpDevMode : * mut DEVMODEW , hwnd : HWND , dwFlags : DWORD , lParam : LPVOID ,) -> LONG ; }
};
}
