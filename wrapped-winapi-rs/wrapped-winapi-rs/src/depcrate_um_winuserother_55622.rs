// Generated macro for other_55622 (other)
macro_rules! Depcrate_um_winuserother_55622 {
() => {
// Module: crate::um::winuser
// Provides: {"other_55622"}
// Dependencies: {}
extern "system" { pub fn ExitWindowsEx (uFlags : UINT , dwReason : DWORD ,) -> BOOL ; pub fn SwapMouseButton (fSwap : BOOL ,) -> BOOL ; pub fn GetMessagePos () -> DWORD ; pub fn GetMessageTime () -> LONG ; pub fn GetMessageExtraInfo () -> LPARAM ; pub fn GetUnpredictedMessagePos () -> DWORD ; pub fn IsWow64Message () -> BOOL ; pub fn SetMessageExtraInfo (lParam : LPARAM ,) -> LPARAM ; pub fn SendMessageA (hWnd : HWND , Msg : UINT , wParam : WPARAM , lParam : LPARAM ,) -> LRESULT ; pub fn SendMessageW (hWnd : HWND , Msg : UINT , wParam : WPARAM , lParam : LPARAM ,) -> LRESULT ; pub fn SendMessageTimeoutA (hWnd : HWND , Msg : UINT , wParam : WPARAM , lParam : LPARAM , fuFlags : UINT , uTimeout : UINT , lpdwResult : PDWORD_PTR ,) -> LRESULT ; pub fn SendMessageTimeoutW (hWnd : HWND , Msg : UINT , wParam : WPARAM , lParam : LPARAM , fuFlags : UINT , uTimeout : UINT , lpdwResult : PDWORD_PTR ,) -> LRESULT ; pub fn SendNotifyMessageA (hWnd : HWND , msg : UINT , wParam : WPARAM , lParam : LPARAM ,) -> BOOL ; pub fn SendNotifyMessageW (hWnd : HWND , msg : UINT , wParam : WPARAM , lParam : LPARAM ,) -> BOOL ; pub fn SendMessageCallbackA (hWnd : HWND , Msg : UINT , wParam : WPARAM , lParam : LPARAM , lpResultCallBack : SENDASYNCPROC , dwData : ULONG_PTR ,) -> BOOL ; pub fn SendMessageCallbackW (hWnd : HWND , Msg : UINT , wParam : WPARAM , lParam : LPARAM , lpResultCallBack : SENDASYNCPROC , dwData : ULONG_PTR ,) -> BOOL ; }
};
}
