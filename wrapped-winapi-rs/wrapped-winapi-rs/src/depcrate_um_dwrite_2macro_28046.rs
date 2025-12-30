// Generated macro for macro_28046 (macro)
macro_rules! Depcrate_um_dwrite_2macro_28046 {
() => {
// Module: crate::um::dwrite_2
// Provides: {"macro_28046"}
// Dependencies: {}
RIDL ! { # [uuid (0x1093c18f , 0x8d5e , 0x43f0 , 0xb0 , 0x64 , 0x09 , 0x17 , 0x31 , 0x1b , 0x52 , 0x5e)] interface IDWriteTextLayout2 (IDWriteTextLayout2Vtbl) : IDWriteTextLayout1 (IDWriteTextLayout1Vtbl) { fn GetMetrics (textMetrics : * mut DWRITE_TEXT_METRICS1 ,) -> HRESULT , fn SetVerticalGlyphOrientation (glyphOrientation : DWRITE_VERTICAL_GLYPH_ORIENTATION ,) -> HRESULT , fn GetVerticalGlyphOrientation () -> DWRITE_VERTICAL_GLYPH_ORIENTATION , fn SetLastLineWrapping (isLastLineWrappingEnabled : BOOL ,) -> HRESULT , fn GetLastLineWrapping () -> BOOL , fn SetOpticalAlignment (opticalAlignment : DWRITE_OPTICAL_ALIGNMENT ,) -> HRESULT , fn GetOpticalAlignment () -> DWRITE_OPTICAL_ALIGNMENT , fn SetFontFallback (fontFallback : * mut IDWriteFontFallback ,) -> HRESULT , fn GetFontFallback (fontFallback : * mut * mut IDWriteFontFallback ,) -> HRESULT , } }
};
}
