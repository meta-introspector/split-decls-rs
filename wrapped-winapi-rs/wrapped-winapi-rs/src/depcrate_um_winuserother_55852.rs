// Generated macro for other_55852 (other)
macro_rules! Depcrate_um_winuserother_55852 {
() => {
// Module: crate::um::winuser
// Provides: {"other_55852"}
// Dependencies: {}
extern "system" { pub fn GetInputState () -> BOOL ; pub fn GetQueueStatus (flags : UINT ,) -> DWORD ; pub fn GetCapture () -> HWND ; pub fn SetCapture (hWnd : HWND ,) -> HWND ; pub fn ReleaseCapture () -> BOOL ; pub fn MsgWaitForMultipleObjects (nCount : DWORD , pHandles : * const HANDLE , fWaitAll : BOOL , dwMilliseconds : DWORD , dwWakeMask : DWORD ,) -> DWORD ; pub fn MsgWaitForMultipleObjectsEx (nCount : DWORD , pHandles : * const HANDLE , dwMilliseconds : DWORD , dwWakeMask : DWORD , dwFlags : DWORD ,) -> DWORD ; }
};
}
