// Generated macro for other_57050 (other)
macro_rules! Depcrate_um_winuserother_57050 {
() => {
// Module: crate::um::winuser
// Provides: {"other_57050"}
// Dependencies: {}
extern "system" { pub fn SetLastErrorEx (dwErrCode : DWORD , dwType : DWORD ,) ; pub fn InternalGetWindowText (hWnd : HWND , pString : LPWSTR , cchMaxCount : c_int ,) -> c_int ; pub fn EndTask (hWnd : HWND , fShutDown : BOOL , fForce : BOOL ,) -> BOOL ; pub fn CancelShutdown () -> BOOL ; }
};
}
