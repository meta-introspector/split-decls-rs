// Generated macro for other_27865 (other)
macro_rules! Depcrate_um_dwmapiother_27865 {
() => {
// Module: crate::um::dwmapi
// Provides: {"other_27865"}
// Dependencies: {}
extern "system" { pub fn DwmEnableComposition (uCompositionAction : UINT ,) -> HRESULT ; pub fn DwmEnableMMCSS (fEnableMMCSS : BOOL ,) -> HRESULT ; pub fn DwmExtendFrameIntoClientArea (hWnd : HWND , pMarInset : * const MARGINS ,) -> HRESULT ; pub fn DwmGetColorizationColor (pcrColorization : * mut DWORD , pfOpaqueBlend : * mut BOOL ,) -> HRESULT ; pub fn DwmGetCompositionTimingInfo (hWnd : HWND , pTimingInfo : * mut DWM_TIMING_INFO ,) -> HRESULT ; pub fn DwmGetWindowAttribute (hWnd : HWND , dwAttribute : DWORD , pvAttribute : LPVOID , cbAttribute : DWORD ,) -> HRESULT ; pub fn DwmIsCompositionEnabled (pfEnabled : * mut BOOL ,) -> HRESULT ; pub fn DwmModifyPreviousDxFrameDuration (hwnd : HWND , cRefreshes : INT , fRelative : BOOL ,) -> HRESULT ; pub fn DwmQueryThumbnailSourceSize (hThumbnail : HTHUMBNAIL , pSize : PSIZE ,) -> HRESULT ; pub fn DwmRegisterThumbnail (hwndDestination : HWND , hwndSource : HWND , phThumbnailId : PHTHUMBNAIL ,) -> HRESULT ; pub fn DwmSetDxFrameDuration (hwnd : HWND , cRefreshes : INT ,) -> HRESULT ; pub fn DwmSetPresentParameters (hwnd : HWND , pPresentParams : * mut DWM_PRESENT_PARAMETERS ,) -> HRESULT ; pub fn DwmSetWindowAttribute (hWnd : HWND , dwAttribute : DWORD , pvAttribute : LPCVOID , cbAttribute : DWORD ,) -> HRESULT ; pub fn DwmUnregisterThumbnail (hThumbnailId : HTHUMBNAIL ,) -> HRESULT ; pub fn DwmUpdateThumbnailProperties (hThumbnailId : HTHUMBNAIL , ptnProperties : * const DWM_THUMBNAIL_PROPERTIES ,) -> HRESULT ; }
};
}
