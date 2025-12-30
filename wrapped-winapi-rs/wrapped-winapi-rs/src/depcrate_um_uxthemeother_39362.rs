// Generated macro for other_39362 (other)
macro_rules! Depcrate_um_uxthemeother_39362 {
() => {
// Module: crate::um::uxtheme
// Provides: {"other_39362"}
// Dependencies: {}
extern "system" { pub fn BeginBufferedPaint (hdcTarget : HDC , prcTarget : * const RECT , dwFormat : BP_BUFFERFORMAT , pPaintParams : * mut BP_PAINTPARAMS , phdc : * mut HDC ,) -> HPAINTBUFFER ; pub fn EndBufferedPaint (hBufferedPaint : HPAINTBUFFER , fUpdateTarget : BOOL ,) -> HRESULT ; pub fn GetBufferedPaintTargetRect (hBufferedPaint : HPAINTBUFFER , prc : * mut RECT ,) -> HRESULT ; pub fn GetBufferedPaintTargetDC (hBufferedPaint : HPAINTBUFFER ,) -> HDC ; pub fn GetBufferedPaintDC (hBufferedPaint : HPAINTBUFFER ,) -> HDC ; pub fn GetBufferedPaintBits (hBufferedPaint : HPAINTBUFFER , ppbBuffer : * mut * mut RGBQUAD , pcxRow : * mut c_int ,) -> HRESULT ; pub fn BufferedPaintClear (hBufferedPaint : HPAINTBUFFER , prc : * const RECT ,) -> HRESULT ; pub fn BufferedPaintSetAlpha (hBufferedPaint : HPAINTBUFFER , prc : * const RECT , alpha : BYTE ,) -> HRESULT ; pub fn BufferedPaintStopAllAnimations (hwnd : HWND ,) -> HRESULT ; }
};
}
