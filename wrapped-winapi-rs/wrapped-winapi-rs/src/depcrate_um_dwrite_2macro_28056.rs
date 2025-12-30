// Generated macro for macro_28056 (macro)
macro_rules! Depcrate_um_dwrite_2macro_28056 {
() => {
// Module: crate::um::dwrite_2
// Provides: {"macro_28056"}
// Dependencies: {}
RIDL ! { # [uuid (0x0439fc60 , 0xca44 , 0x4994 , 0x8d , 0xee , 0x3a , 0x9a , 0xf7 , 0xb7 , 0x32 , 0xec)] interface IDWriteFactory2 (IDWriteFactory2Vtbl) : IDWriteFactory1 (IDWriteFactory1Vtbl) { fn GetSystemFontFallback (fontFallback : * mut * mut IDWriteFontFallback ,) -> HRESULT , fn CreateFontFallbackBuilder (fontFallbackBuilder : * mut * mut IDWriteFontFallbackBuilder ,) -> HRESULT , fn TranslateColorGlyphRun (baselineOriginX : FLOAT , baselineOriginY : FLOAT , glyphRun : * const DWRITE_GLYPH_RUN , glyphRunDescription : * const DWRITE_GLYPH_RUN_DESCRIPTION , measuringMode : DWRITE_MEASURING_MODE , worldToDeviceTransform : * const DWRITE_MATRIX , colorPaletteIndex : UINT32 , colorLayers : * mut * mut IDWriteColorGlyphRunEnumerator ,) -> HRESULT , fn CreateCustomRenderingParams (gamma : FLOAT , enhancedContrast : FLOAT , grayscaleEnhancedContrast : FLOAT , clearTypeLevel : FLOAT , pixelGeometry : DWRITE_PIXEL_GEOMETRY , renderingMode : DWRITE_RENDERING_MODE , gridFitMode : DWRITE_GRID_FIT_MODE , renderingParams : * mut * mut IDWriteRenderingParams2 ,) -> HRESULT , fn CreateGlyphRunAnalysis (glyphRun : * const DWRITE_GLYPH_RUN , transform : * const DWRITE_MATRIX , renderingMode : DWRITE_RENDERING_MODE , measuringMode : DWRITE_MEASURING_MODE , gridFitMode : DWRITE_GRID_FIT_MODE , antialiasMode : DWRITE_TEXT_ANTIALIAS_MODE , baselineOriginX : FLOAT , baselineOriginY : FLOAT , glyphRunAnalysis : * mut * mut IDWriteGlyphRunAnalysis ,) -> HRESULT , } }
};
}
