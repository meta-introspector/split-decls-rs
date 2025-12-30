// Generated macro for other_27867 (other)
macro_rules! Depcrate_um_dwmapiother_27867 {
() => {
// Module: crate::um::dwmapi
// Provides: {"other_27867"}
// Dependencies: {}
extern "system" { pub fn DwmSetIconicThumbnail (hwnd : HWND , hbmp : HBITMAP , dwSITFlags : DWORD ,) -> HRESULT ; pub fn DwmSetIconicLivePreviewBitmap (hwnd : HWND , hbmp : HBITMAP , pptClient : * mut POINT , dwSITFlags : DWORD ,) -> HRESULT ; pub fn DwmInvalidateIconicBitmaps (hwnd : HWND ,) -> HRESULT ; pub fn DwmFlush () -> HRESULT ; pub fn DwmGetTransportAttributes (pfIsRemoting : * mut BOOL , pfIsConnected : * mut BOOL , pDwGeneration : * mut DWORD ,) -> HRESULT ; }
};
}
