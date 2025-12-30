// Generated macro for other_55652 (other)
macro_rules! Depcrate_um_winuserother_55652 {
() => {
// Module: crate::um::winuser
// Provides: {"other_55652"}
// Dependencies: {}
extern "system" { pub fn RegisterPowerSettingNotification (hRecipient : HANDLE , PowerSettingGuid : LPCGUID , Flags : DWORD ,) -> HPOWERNOTIFY ; pub fn UnregisterPowerSettingNotification (Handle : HPOWERNOTIFY ,) -> BOOL ; pub fn RegisterSuspendResumeNotification (hRecipient : HANDLE , Flags : DWORD ,) -> HPOWERNOTIFY ; pub fn UnregisterSuspendResumeNotification (Handle : HPOWERNOTIFY ,) -> BOOL ; pub fn PostMessageA (hWnd : HWND , Msg : UINT , wParam : WPARAM , lParam : LPARAM ,) -> BOOL ; pub fn PostMessageW (hWnd : HWND , Msg : UINT , wParam : WPARAM , lParam : LPARAM ,) -> BOOL ; pub fn PostThreadMessageA (idThread : DWORD , msg : UINT , wParam : WPARAM , lParam : LPARAM ,) -> BOOL ; pub fn PostThreadMessageW (idThread : DWORD , msg : UINT , wParam : WPARAM , lParam : LPARAM ,) -> BOOL ; }
};
}
