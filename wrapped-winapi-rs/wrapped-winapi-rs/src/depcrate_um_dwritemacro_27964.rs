// Generated macro for macro_27964 (macro)
macro_rules! Depcrate_um_dwritemacro_27964 {
() => {
// Module: crate::um::dwrite
// Provides: {"macro_27964"}
// Dependencies: {}
RIDL ! { # [uuid (0x1edd9491 , 0x9853 , 0x4299 , 0x89 , 0x8f , 0x64 , 0x32 , 0x98 , 0x3b , 0x6f , 0x3a)] interface IDWriteGdiInterop (IDWriteGdiInteropVtbl) : IUnknown (IUnknownVtbl) { fn CreateFontFromLOGFONT (logFont : * const LOGFONTW , font : * mut * mut IDWriteFont ,) -> HRESULT , fn ConvertFontToLOGFONT (font : * mut IDWriteFont , logFont : * mut LOGFONTW , isSystemFont : * mut BOOL ,) -> HRESULT , fn ConvertFontFaceToLOGFONT (font : * mut IDWriteFontFace , logFont : * mut LOGFONTW ,) -> HRESULT , fn CreateFontFaceFromHdc (hdc : HDC , fontFace : * mut * mut IDWriteFontFace ,) -> HRESULT , fn CreateBitmapRenderTarget (hdc : HDC , width : UINT32 , height : UINT32 , renderTarget : * mut * mut IDWriteBitmapRenderTarget ,) -> HRESULT , } }
};
}
