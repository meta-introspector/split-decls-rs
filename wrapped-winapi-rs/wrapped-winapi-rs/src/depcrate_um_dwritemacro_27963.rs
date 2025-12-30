// Generated macro for macro_27963 (macro)
macro_rules! Depcrate_um_dwritemacro_27963 {
() => {
// Module: crate::um::dwrite
// Provides: {"macro_27963"}
// Dependencies: {}
RIDL ! { # [uuid (0x5e5a32a3 , 0x8dff , 0x4773 , 0x9f , 0xf6 , 0x06 , 0x96 , 0xea , 0xb7 , 0x72 , 0x67)] interface IDWriteBitmapRenderTarget (IDWriteBitmapRenderTargetVtbl) : IUnknown (IUnknownVtbl) { fn DrawGlyphRun (baselineOriginX : FLOAT , baselineOriginY : FLOAT , measuringMode : DWRITE_MEASURING_MODE , glyphRun : * const DWRITE_GLYPH_RUN , renderingParams : * mut IDWriteRenderingParams , textColor : COLORREF , blackBoxRect : * mut RECT ,) -> HRESULT , fn GetMemoryDC () -> HDC , fn GetPixelsPerDip () -> FLOAT , fn SetPixelsPerDip (pixelsPerDip : FLOAT ,) -> HRESULT , fn GetCurrentTransform (transform : * mut DWRITE_MATRIX ,) -> HRESULT , fn SetCurrentTransform (transform : * const DWRITE_MATRIX ,) -> HRESULT , fn GetSize (size : * mut SIZE ,) -> HRESULT , fn Resize (width : UINT32 , height : UINT32 ,) -> HRESULT , } }
};
}
