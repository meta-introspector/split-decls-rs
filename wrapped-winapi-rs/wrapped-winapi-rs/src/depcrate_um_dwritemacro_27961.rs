// Generated macro for macro_27961 (macro)
macro_rules! Depcrate_um_dwritemacro_27961 {
() => {
// Module: crate::um::dwrite
// Provides: {"macro_27961"}
// Dependencies: {}
RIDL ! { # [uuid (0xef8a8135 , 0x5cc6 , 0x45fe , 0x88 , 0x25 , 0xc5 , 0xa0 , 0x72 , 0x4e , 0xb8 , 0x19)] interface IDWriteTextRenderer (IDWriteTextRendererVtbl) : IDWritePixelSnapping (IDWritePixelSnappingVtbl) { fn DrawGlyphRun (clientDrawingContext : * mut c_void , baselineOriginX : FLOAT , baselineOriginY : FLOAT , measuringMode : DWRITE_MEASURING_MODE , glyphRun : * const DWRITE_GLYPH_RUN , glyphRunDescription : * const DWRITE_GLYPH_RUN_DESCRIPTION , clientDrawingEffect : * mut IUnknown ,) -> HRESULT , fn DrawUnderline (clientDrawingContext : * mut c_void , baselineOriginX : FLOAT , baselineOriginY : FLOAT , underline : * const DWRITE_UNDERLINE , clientDrawingEffect : * mut IUnknown ,) -> HRESULT , fn DrawStrikethrough (clientDrawingContext : * mut c_void , baselineOriginX : FLOAT , baselineOriginY : FLOAT , strikethrough : * const DWRITE_STRIKETHROUGH , clientDrawingEffect : * mut IUnknown ,) -> HRESULT , fn DrawInlineObject (clientDrawingContext : * mut c_void , baselineOriginX : FLOAT , baselineOriginY : FLOAT , inlineObject : * mut IDWriteInlineObject , isSideways : BOOL , isRightToLeft : BOOL , clientDrawingEffect : * mut IUnknown ,) -> HRESULT , } }
};
}
