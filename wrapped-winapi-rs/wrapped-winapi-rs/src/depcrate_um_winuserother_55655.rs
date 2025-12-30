// Generated macro for other_55655 (other)
macro_rules! Depcrate_um_winuserother_55655 {
() => {
// Module: crate::um::winuser
// Provides: {"other_55655"}
// Dependencies: {}
extern "system" { pub fn AttachThreadInput (idAttach : DWORD , idAttachTo : DWORD , fAttach : BOOL ,) -> BOOL ; pub fn ReplyMessage (lResult : LRESULT ,) -> BOOL ; pub fn WaitMessage () -> BOOL ; pub fn WaitForInputIdle (hProcess : HANDLE , dwMilliseconds : DWORD ,) -> DWORD ; pub fn DefWindowProcA (hWnd : HWND , Msg : UINT , wParam : WPARAM , lParam : LPARAM ,) -> LRESULT ; pub fn DefWindowProcW (hWnd : HWND , Msg : UINT , wParam : WPARAM , lParam : LPARAM ,) -> LRESULT ; pub fn PostQuitMessage (nExitCode : c_int ,) ; pub fn CallWindowProcA (lpPrevWndFunc : WNDPROC , hWnd : HWND , Msg : UINT , wParam : WPARAM , lParam : LPARAM ,) -> LRESULT ; pub fn CallWindowProcW (lpPrevWndFunc : WNDPROC , hWnd : HWND , Msg : UINT , wParam : WPARAM , lParam : LPARAM ,) -> LRESULT ; pub fn InSendMessage () -> BOOL ; pub fn InSendMessageEx (lpReserved : LPVOID ,) -> DWORD ; }
};
}
