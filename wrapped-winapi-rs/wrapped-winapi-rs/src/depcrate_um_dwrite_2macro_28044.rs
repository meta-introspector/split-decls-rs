// Generated macro for macro_28044 (macro)
macro_rules! Depcrate_um_dwrite_2macro_28044 {
() => {
// Module: crate::um::dwrite_2
// Provides: {"macro_28044"}
// Dependencies: {}
RIDL ! { # [uuid (0xd3e0e934 , 0x22a0 , 0x427e , 0xaa , 0xe4 , 0x7d , 0x95 , 0x74 , 0xb5 , 0x9d , 0xb1)] interface IDWriteTextRenderer1 (IDWriteTextRenderer1Vtbl) : IDWriteTextRenderer (IDWriteTextRendererVtbl) { fn DrawGlyphRun (clientDrawingContext : * mut c_void , baselineOriginX : FLOAT , baselineOriginY : FLOAT , orientationAngle : DWRITE_GLYPH_ORIENTATION_ANGLE , measuringMode : DWRITE_MEASURING_MODE , glyphRun : * const DWRITE_GLYPH_RUN , glyphRunDescription : * const DWRITE_GLYPH_RUN_DESCRIPTION , clientDrawingEffect : * mut IUnknown ,) -> HRESULT , fn DrawUnderline (clientDrawingContext : * mut c_void , baselineOriginX : FLOAT , baselineOriginY : FLOAT , orientationAngle : DWRITE_GLYPH_ORIENTATION_ANGLE , underline : * const DWRITE_UNDERLINE , clientDrawingEffect : * mut IUnknown ,) -> HRESULT , fn DrawStrikethrough (clientDrawingContext : * mut c_void , baselineOriginX : FLOAT , baselineOriginY : FLOAT , orientationAngle : DWRITE_GLYPH_ORIENTATION_ANGLE , strikethrough : * const DWRITE_STRIKETHROUGH , clientDrawingEffect : * mut IUnknown ,) -> HRESULT , fn DrawInlineObject (clientDrawingContext : * mut c_void , originX : FLOAT , originY : FLOAT , orientationAngle : DWRITE_GLYPH_ORIENTATION_ANGLE , inlineObject : * mut IDWriteInlineObject , isSideways : BOOL , isRightToLeft : BOOL , clientDrawingEffect : * mut IUnknown ,) -> HRESULT , } }
};
}
