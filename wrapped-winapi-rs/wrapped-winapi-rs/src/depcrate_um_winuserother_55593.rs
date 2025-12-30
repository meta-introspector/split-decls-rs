// Generated macro for other_55593 (other)
macro_rules! Depcrate_um_winuserother_55593 {
() => {
// Module: crate::um::winuser
// Provides: {"other_55593"}
// Dependencies: {}
extern "system" { pub fn GetMessageA (lpMsg : LPMSG , hWnd : HWND , wMsgFilterMin : UINT , wMsgFilterMax : UINT ,) -> BOOL ; pub fn GetMessageW (lpMsg : LPMSG , hWnd : HWND , wMsgFilterMin : UINT , wMsgFilterMax : UINT ,) -> BOOL ; pub fn TranslateMessage (lpmsg : * const MSG ,) -> BOOL ; pub fn DispatchMessageA (lpmsg : * const MSG ,) -> LRESULT ; pub fn DispatchMessageW (lpmsg : * const MSG ,) -> LRESULT ; pub fn SetMessageQueue (cMessagesMax : c_int ,) -> BOOL ; pub fn PeekMessageA (lpMsg : LPMSG , hWnd : HWND , wMsgFilterMin : UINT , wMsgFilterMax : UINT , wRemoveMsg : UINT ,) -> BOOL ; pub fn PeekMessageW (lpMsg : LPMSG , hWnd : HWND , wMsgFilterMin : UINT , wMsgFilterMax : UINT , wRemoveMsg : UINT ,) -> BOOL ; }
};
}
