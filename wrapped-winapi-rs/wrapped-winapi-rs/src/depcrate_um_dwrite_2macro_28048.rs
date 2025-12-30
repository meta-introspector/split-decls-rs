// Generated macro for macro_28048 (macro)
macro_rules! Depcrate_um_dwrite_2macro_28048 {
() => {
// Module: crate::um::dwrite_2
// Provides: {"macro_28048"}
// Dependencies: {}
RIDL ! { # [uuid (0xefa008f9 , 0xf7a1 , 0x48bf , 0xb0 , 0x5c , 0xf2 , 0x24 , 0x71 , 0x3c , 0xc0 , 0xff)] interface IDWriteFontFallback (IDWriteFontFallbackVtbl) : IUnknown (IUnknownVtbl) { fn MapCharacters (analysisSource : * mut IDWriteTextAnalysisSource , textPosition : UINT32 , textLength : UINT32 , baseFontCollection : * mut IDWriteFontCollection , baseFamilyName : * mut wchar_t , baseWeight : DWRITE_FONT_WEIGHT , baseStyle : DWRITE_FONT_STYLE , baseStretch : DWRITE_FONT_STRETCH , mappedLength : * mut UINT32 , mappedFont : * mut * mut IDWriteFont , scale : * mut FLOAT ,) -> HRESULT , } }
};
}
