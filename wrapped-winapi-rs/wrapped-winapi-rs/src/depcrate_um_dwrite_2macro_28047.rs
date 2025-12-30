// Generated macro for macro_28047 (macro)
macro_rules! Depcrate_um_dwrite_2macro_28047 {
() => {
// Module: crate::um::dwrite_2
// Provides: {"macro_28047"}
// Dependencies: {}
RIDL ! { # [uuid (0x553a9ff3 , 0x5693 , 0x4df7 , 0xb5 , 0x2b , 0x74 , 0x80 , 0x6f , 0x7f , 0x2e , 0xb9)] interface IDWriteTextAnalyzer2 (IDWriteTextAnalyzer2Vtbl) : IDWriteTextAnalyzer1 (IDWriteTextAnalyzer1Vtbl) { fn GetGlyphOrientationTransform (glyphOrientationAngle : DWRITE_GLYPH_ORIENTATION_ANGLE , isSideways : BOOL , originX : FLOAT , originY : FLOAT , transform : * mut DWRITE_MATRIX ,) -> HRESULT , fn GetTypographicFeatures (fontFace : * mut IDWriteFontFace , scriptAnalysis : DWRITE_SCRIPT_ANALYSIS , localeName : * const WCHAR , maxTagCount : UINT32 , actualTagCount : * mut UINT32 , tags : * mut DWRITE_FONT_FEATURE_TAG ,) -> HRESULT , fn CheckTypographicFeature (fontFace : * mut IDWriteFontFace , scriptAnalysis : DWRITE_SCRIPT_ANALYSIS , localeName : * const WCHAR , featureTag : DWRITE_FONT_FEATURE_TAG , glyphCount : UINT32 , glyphIndices : * const UINT16 , featureApplies : * mut UINT8 ,) -> HRESULT , } }
};
}
