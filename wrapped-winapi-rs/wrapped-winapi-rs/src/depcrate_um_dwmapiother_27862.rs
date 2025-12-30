// Generated macro for other_27862 (other)
macro_rules! Depcrate_um_dwmapiother_27862 {
() => {
// Module: crate::um::dwmapi
// Provides: {"other_27862"}
// Dependencies: {}
extern "system" { pub fn DwmDefWindowProc (hWnd : HWND , msg : UINT , wParam : WPARAM , lParam : LPARAM , plResult : * mut LRESULT ,) -> BOOL ; pub fn DwmEnableBlurBehindWindow (hWnd : HWND , pBlurBehind : * const DWM_BLURBEHIND ,) -> HRESULT ; }
};
}
