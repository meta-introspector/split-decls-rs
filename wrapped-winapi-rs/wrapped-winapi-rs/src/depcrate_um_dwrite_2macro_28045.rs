// Generated macro for macro_28045 (macro)
macro_rules! Depcrate_um_dwrite_2macro_28045 {
() => {
// Module: crate::um::dwrite_2
// Provides: {"macro_28045"}
// Dependencies: {}
RIDL ! { # [uuid (0x5f174b49 , 0x0d8b , 0x4cfb , 0x8b , 0xca , 0xf1 , 0xcc , 0xe9 , 0xd0 , 0x6c , 0x67)] interface IDWriteTextFormat1 (IDWriteTextFormat1Vtbl) : IDWriteTextFormat (IDWriteTextFormatVtbl) { fn SetVerticalGlyphOrientation (glyphOrientation : DWRITE_VERTICAL_GLYPH_ORIENTATION ,) -> HRESULT , fn GetVerticalGlyphOrientation () -> DWRITE_VERTICAL_GLYPH_ORIENTATION , fn SetLastLineWrapping (isLastLineWrappingEnabled : BOOL ,) -> HRESULT , fn GetLastLineWrapping () -> BOOL , fn SetOpticalAlignment (opticalAlignment : DWRITE_OPTICAL_ALIGNMENT ,) -> HRESULT , fn GetOpticalAlignment () -> DWRITE_OPTICAL_ALIGNMENT , fn SetFontFallback (fontFallback : * mut IDWriteFontFallback ,) -> HRESULT , fn GetFontFallback (fontFallback : * mut * mut IDWriteFontFallback ,) -> HRESULT , } }
};
}
