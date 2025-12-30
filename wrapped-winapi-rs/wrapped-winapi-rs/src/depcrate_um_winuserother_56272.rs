// Generated macro for other_56272 (other)
macro_rules! Depcrate_um_winuserother_56272 {
() => {
// Module: crate::um::winuser
// Provides: {"other_56272"}
// Dependencies: {}
extern "system" { pub fn GetProcessDefaultLayout (pdwDefaultLayout : * mut DWORD ,) -> BOOL ; pub fn SetProcessDefaultLayout (dwDefaultLayout : DWORD ,) -> BOOL ; pub fn GetDesktopWindow () -> HWND ; pub fn GetParent (hWnd : HWND ,) -> HWND ; pub fn SetParent (hWndChild : HWND , hWndNewParent : HWND ,) -> HWND ; pub fn EnumChildWindows (hWndParent : HWND , lpEnumFunc : WNDENUMPROC , lParam : LPARAM ,) -> BOOL ; pub fn FindWindowA (lpClassName : LPCSTR , lpWindowName : LPCSTR ,) -> HWND ; pub fn FindWindowW (lpClassName : LPCWSTR , lpWindowName : LPCWSTR ,) -> HWND ; pub fn FindWindowExA (hWndParent : HWND , hWndChildAfter : HWND , lpszClass : LPCSTR , lpszWindow : LPCSTR ,) -> HWND ; pub fn FindWindowExW (hWndParent : HWND , hWndChildAfter : HWND , lpszClass : LPCWSTR , lpszWindow : LPCWSTR ,) -> HWND ; pub fn GetShellWindow () -> HWND ; pub fn RegisterShellHookWindow (hwnd : HWND ,) -> BOOL ; pub fn DeregisterShellHookWindow (hwnd : HWND ,) -> BOOL ; pub fn EnumWindows (lpEnumFunc : WNDENUMPROC , lParam : LPARAM ,) -> BOOL ; pub fn EnumThreadWindows (dwThreadId : DWORD , lpfn : WNDENUMPROC , lParam : LPARAM ,) -> BOOL ; }
};
}
