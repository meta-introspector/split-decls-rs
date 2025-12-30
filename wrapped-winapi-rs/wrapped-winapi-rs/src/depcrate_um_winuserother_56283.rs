// Generated macro for other_56283 (other)
macro_rules! Depcrate_um_winuserother_56283 {
() => {
// Module: crate::um::winuser
// Provides: {"other_56283"}
// Dependencies: {}
extern "system" { pub fn GetWindow (hWnd : HWND , uCmd : UINT ,) -> HWND ; pub fn SetWindowsHookA (nFilterType : c_int , pfnFilterProc : HOOKPROC ,) -> HHOOK ; pub fn SetWindowsHookW (nFilterType : c_int , pfnFilterProc : HOOKPROC ,) -> HHOOK ; pub fn UnhookWindowsHook (nFilterType : c_int , pfnFilterProc : HOOKPROC ,) -> BOOL ; pub fn SetWindowsHookExA (idHook : c_int , lpfn : HOOKPROC , hmod : HINSTANCE , dwThreadId : DWORD ,) -> HHOOK ; pub fn SetWindowsHookExW (idHook : c_int , lpfn : HOOKPROC , hmod : HINSTANCE , dwThreadId : DWORD ,) -> HHOOK ; pub fn UnhookWindowsHookEx (hhk : HHOOK ,) -> BOOL ; pub fn CallNextHookEx (hhk : HHOOK , nCode : c_int , wParam : WPARAM , lParam : LPARAM ,) -> LRESULT ; }
};
}
