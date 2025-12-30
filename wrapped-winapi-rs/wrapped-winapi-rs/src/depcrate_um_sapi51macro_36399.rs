// Generated macro for macro_36399 (macro)
macro_rules! Depcrate_um_sapi51macro_36399 {
() => {
// Module: crate::um::sapi51
// Provides: {"macro_36399"}
// Dependencies: {}
RIDL ! { # [uuid (0x5eff4aef , 0x8487 , 0x11d2 , 0x96 , 0x1c , 0x00 , 0xc0 , 0x4f , 0x8e , 0xe6 , 0x28)] interface ISpNotifySource (ISpNotifySourceVtbl) : IUnknown (IUnknownVtbl) { fn SetNotifySink (pNotifySink : * mut ISpNotifySink ,) -> HRESULT , fn SetNotifyWindowMessage (hWnd : HWND , Msg : UINT , wParam : WPARAM , lParam : LPARAM ,) -> HRESULT , fn SetNotifyCallbackFunction (pfnCallback : SPNOTIFYCALLBACK , wParam : WPARAM , lParam : LPARAM ,) -> HRESULT , fn SetNotifyCallbackInterface (pSpCallback : * mut ISpNotifyCallback , wParam : WPARAM , lParam : LPARAM ,) -> HRESULT , fn SetNotifyWin32Event () -> HRESULT , fn WaitForNotifyEvent (dwMilliseconds : DWORD ,) -> HRESULT , fn GetNotifyEventHandle () -> HANDLE , } }
};
}
