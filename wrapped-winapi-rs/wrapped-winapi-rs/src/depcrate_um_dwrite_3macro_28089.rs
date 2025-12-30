// Generated macro for macro_28089 (macro)
macro_rules! Depcrate_um_dwrite_3macro_28089 {
() => {
// Module: crate::um::dwrite_3
// Provides: {"macro_28089"}
// Dependencies: {}
RIDL ! { # [uuid (0x4556be70 , 0x3abd , 0x4f70 , 0x90 , 0xbe , 0x42 , 0x17 , 0x80 , 0xa6 , 0xf5 , 0x15)] interface IDWriteGdiInterop1 (IDWriteGdiInterop1Vtbl) : IDWriteGdiInterop (IDWriteGdiInteropVtbl) { fn CreateFontFromLOGFONT (logFont : * const LOGFONTW , fontCollection : * mut IDWriteFontCollection , font : * mut * mut IDWriteFont ,) -> HRESULT , fn GetFontSignature_2 (fontFace : * mut IDWriteFontFace , fontSignature : * mut FONTSIGNATURE ,) -> HRESULT , fn GetFontSignature_1 (font : * mut IDWriteFont , fontSignature : * mut FONTSIGNATURE ,) -> HRESULT , fn GetMatchingFontsByLOGFONT (logFont : * const LOGFONTW , fontSet : * mut IDWriteFontSet , filteredSet : * mut * mut IDWriteFontSet ,) -> HRESULT , } }
};
}
