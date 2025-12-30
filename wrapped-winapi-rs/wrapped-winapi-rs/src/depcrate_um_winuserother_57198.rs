// Generated macro for other_57198 (other)
macro_rules! Depcrate_um_winuserother_57198 {
() => {
// Module: crate::um::winuser
// Provides: {"other_57198"}
// Dependencies: {}
extern "system" { pub fn SetProcessDPIAware () -> BOOL ; pub fn IsProcessDPIAware () -> BOOL ; pub fn SetThreadDpiAwarenessContext (dpiContext : DPI_AWARENESS_CONTEXT ,) -> DPI_AWARENESS_CONTEXT ; pub fn GetThreadDpiAwarenessContext () -> DPI_AWARENESS_CONTEXT ; pub fn GetWindowDpiAwarenessContext (hwnd : HWND ,) -> DPI_AWARENESS_CONTEXT ; pub fn GetAwarenessFromDpiAwarenessContext (value : DPI_AWARENESS_CONTEXT ,) -> DPI_AWARENESS ; pub fn GetDpiFromDpiAwarenessContext (value : DPI_AWARENESS_CONTEXT ,) -> UINT ; pub fn AreDpiAwarenessContextsEqual (dpiContextA : DPI_AWARENESS_CONTEXT , dpiContextB : DPI_AWARENESS_CONTEXT ,) -> BOOL ; pub fn IsValidDpiAwarenessContext (value : DPI_AWARENESS_CONTEXT ,) -> BOOL ; pub fn GetDpiForWindow (hwnd : HWND ,) -> UINT ; pub fn GetDpiForSystem () -> UINT ; pub fn GetSystemDpiForProcess (hProcess : HANDLE ,) -> UINT ; pub fn EnableNonClientDpiScaling (hwnd : HWND ,) -> BOOL ; pub fn SetProcessDpiAwarenessContext (value : DPI_AWARENESS_CONTEXT ,) -> BOOL ; pub fn SetThreadDpiHostingBehavior (value : DPI_HOSTING_BEHAVIOR ,) -> DPI_HOSTING_BEHAVIOR ; pub fn GetThreadDpiHostingBehavior () -> DPI_HOSTING_BEHAVIOR ; pub fn GetWindowDpiHostingBehavior (hwnd : HWND ,) -> DPI_HOSTING_BEHAVIOR ; pub fn GetWindowModuleFileNameA (hWnd : HWND , lpszFileName : LPCSTR , cchFileNameMax : UINT ,) -> UINT ; pub fn GetWindowModuleFileNameW (hWnd : HWND , lpszFileName : LPWSTR , cchFileNameMax : UINT ,) -> UINT ; }
};
}
