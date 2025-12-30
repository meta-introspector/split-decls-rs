// Generated macro for other_55879 (other)
macro_rules! Depcrate_um_winuserother_55879 {
() => {
// Module: crate::um::winuser
// Provides: {"other_55879"}
// Dependencies: {}
extern "system" { pub fn SetCoalescableTimer (hWnd : HWND , nIDEvent : UINT_PTR , uElapse : UINT , lpTimerFunc : TIMERPROC , uToleranceDelay : ULONG ,) -> UINT_PTR ; pub fn KillTimer (hWnd : HWND , uIDEvent : UINT_PTR ,) -> BOOL ; pub fn IsWindowUnicode (hWnd : HWND ,) -> BOOL ; pub fn EnableWindow (hWnd : HWND , bEnable : BOOL ,) -> BOOL ; pub fn IsWindowEnabled (hWnd : HWND ,) -> BOOL ; pub fn LoadAcceleratorsA (hInstance : HINSTANCE , lpTableName : LPCSTR ,) -> HACCEL ; pub fn LoadAcceleratorsW (hInstance : HINSTANCE , lpTableName : LPCWSTR ,) -> HACCEL ; pub fn CreateAcceleratorTableA (paccel : LPACCEL , cAccel : c_int ,) -> HACCEL ; pub fn CreateAcceleratorTableW (paccel : LPACCEL , cAccel : c_int ,) -> HACCEL ; pub fn DestroyAcceleratorTable (hAccel : HACCEL ,) -> BOOL ; pub fn CopyAcceleratorTableA (hAccelSrc : HACCEL , lpAccelDst : LPACCEL , cAccelEntries : c_int ,) -> c_int ; pub fn CopyAcceleratorTableW (hAccelSrc : HACCEL , lpAccelDst : LPACCEL , cAccelEntries : c_int ,) -> c_int ; pub fn TranslateAcceleratorA (hWnd : HWND , hAccTable : HACCEL , lpMsg : LPMSG ,) -> c_int ; pub fn TranslateAcceleratorW (hWnd : HWND , hAccTable : HACCEL , lpMsg : LPMSG ,) -> c_int ; }
};
}
