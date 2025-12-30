// Generated macro for other_39364 (other)
macro_rules! Depcrate_um_uxthemeother_39364 {
() => {
// Module: crate::um::uxtheme
// Provides: {"other_39364"}
// Dependencies: {}
extern "system" { pub fn BeginBufferedAnimation (hwnd : HWND , hdcTarget : HDC , prcTarget : * const RECT , dwFormat : BP_BUFFERFORMAT , pPaintParams : * mut BP_PAINTPARAMS , pAnimationParams : * mut BP_ANIMATIONPARAMS , phdcFrom : * mut HDC , phdcTo : * mut HDC ,) -> HANIMATIONBUFFER ; pub fn EndBufferedAnimation (hbpAnimation : HANIMATIONBUFFER , fUpdateTarget : BOOL ,) -> HRESULT ; pub fn BufferedPaintRenderAnimation (hwnd : HWND , hdcTarget : HDC ,) -> BOOL ; pub fn IsCompositionActive () -> BOOL ; pub fn GetThemeTransitionDuration (hTheme : HTHEME , iPartId : c_int , iStateIdFrom : c_int , iStateIdTo : c_int , iPropId : c_int , pdwDuration : * mut DWORD ,) -> HRESULT ; }
};
}
