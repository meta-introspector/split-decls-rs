// Generated macro for other_57372 (other)
macro_rules! Depcrate_um_winuserother_57372 {
() => {
// Module: crate::um::winuser
// Provides: {"other_57372"}
// Dependencies: {}
extern "system" { pub fn ShutdownBlockReasonCreate (hWnd : HWND , pwszReason : LPCWSTR ,) -> BOOL ; pub fn ShutdownBlockReasonQuery (hWnd : HWND , pwszBuff : LPWSTR , pcchBuff : * mut DWORD ,) -> BOOL ; pub fn ShutdownBlockReasonDestroy (hWnd : HWND ,) -> BOOL ; }
};
}
