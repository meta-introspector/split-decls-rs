// Generated macro for other_56116 (other)
macro_rules! Depcrate_um_winuserother_56116 {
() => {
// Module: crate::um::winuser
// Provides: {"other_56116"}
// Dependencies: {}
extern "system" { pub fn DrawStateA (hdc : HDC , hbrFore : HBRUSH , qfnCallBack : DRAWSTATEPROC , lData : LPARAM , wData : WPARAM , x : c_int , y : c_int , cx : c_int , cy : c_int , uFlags : UINT ,) -> BOOL ; pub fn DrawStateW (hdc : HDC , hbrFore : HBRUSH , qfnCallBack : DRAWSTATEPROC , lData : LPARAM , wData : WPARAM , x : c_int , y : c_int , cx : c_int , cy : c_int , uFlags : UINT ,) -> BOOL ; pub fn TabbedTextOutA (hdc : HDC , x : c_int , y : c_int , lpString : LPCSTR , chCount : c_int , nTabPositions : c_int , lpnTabStopPositions : * const INT , nTabOrigin : c_int ,) -> LONG ; pub fn TabbedTextOutW (hdc : HDC , x : c_int , y : c_int , lpString : LPCWSTR , chCount : c_int , nTabPositions : c_int , lpnTabStopPositions : * const INT , nTabOrigin : c_int ,) -> LONG ; pub fn GetTabbedTextExtentA (hdc : HDC , lpString : LPCSTR , chCount : c_int , nTabPositions : c_int , lpnTabStopPositions : * const INT ,) -> DWORD ; pub fn GetTabbedTextExtentW (hdc : HDC , lpString : LPCWSTR , chCount : c_int , nTabPositions : c_int , lpnTabStopPositions : * const INT ,) -> DWORD ; pub fn UpdateWindow (hWnd : HWND ,) -> BOOL ; pub fn SetActiveWindow (hWnd : HWND ,) -> HWND ; pub fn GetForegroundWindow () -> HWND ; pub fn PaintDesktop (hdc : HDC ,) -> BOOL ; pub fn SwitchToThisWindow (hwnd : HWND , fUnknown : BOOL ,) ; pub fn SetForegroundWindow (hWnd : HWND ,) -> BOOL ; pub fn AllowSetForegroundWindow (dwProcessId : DWORD ,) -> BOOL ; }
};
}
