// Generated macro for macro_36401 (macro)
macro_rules! Depcrate_um_sapi51macro_36401 {
() => {
// Module: crate::um::sapi51
// Provides: {"macro_36401"}
// Dependencies: {}
RIDL ! { # [uuid (0xaca16614 , 0x5d3d , 0x11d2 , 0x96 , 0x0e , 0x00 , 0xc0 , 0x4f , 0x8e , 0xe6 , 0x28)] interface ISpNotifyTranslator (ISpNotifyTranslatorVtbl) : ISpNotifySink (ISpNotifySinkVtbl) { fn InitWindowMessage (hWnd : HWND , Msg : UINT , wParam : WPARAM , lParam : LPARAM ,) -> HRESULT , fn InitCallback (pfnCallback : SPNOTIFYCALLBACK , wParam : WPARAM , lParam : LPARAM ,) -> HRESULT , fn InitSpNotifyCallback (pSpCallback : * mut ISpNotifyCallback , wParam : WPARAM , lParam : LPARAM ,) -> HRESULT , fn InitWin32Event (hEvent : HANDLE , fCloseHandleOnRelease : BOOL ,) -> HRESULT , fn Wait (dwMilliseconds : DWORD ,) -> HRESULT , fn GetEventHandle () -> HANDLE , } }
};
}
