// Generated macro for macro_36854 (macro)
macro_rules! Depcrate_um_sapiddk51macro_36854 {
() => {
// Module: crate::um::sapiddk51
// Provides: {"macro_36854"}
// Dependencies: {}
RIDL ! { # [uuid (0xa6be4d73 , 0x4403 , 0x4358 , 0xb2 , 0x2d , 0x03 , 0x46 , 0xe2 , 0x3b , 0x17 , 0x64)] interface ISpThreadControl (ISpThreadControlVtbl) : ISpNotifySink (ISpNotifySinkVtbl) { fn StartThread (dwFlags : DWORD , phwnd : * mut HWND ,) -> HRESULT , fn WaitForThreadDone (fForceStop : BOOL , phrThreadResult : * mut HRESULT , msTimeOut : ULONG ,) -> HRESULT , fn TerminateThread () -> HRESULT , fn ThreadHandle () -> HANDLE , fn ThreadId () -> DWORD , fn NotifyEvent () -> HANDLE , fn WindowHandle () -> HWND , fn ThreadCompleteEvent () -> HANDLE , fn ExitThreadEvent () -> HANDLE , } }
};
}
