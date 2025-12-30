// Generated macro for macro_27959 (macro)
macro_rules! Depcrate_um_dwritemacro_27959 {
() => {
// Module: crate::um::dwrite
// Provides: {"macro_27959"}
// Dependencies: {}
RIDL ! { # [uuid (0x8339fde3 , 0x106f , 0x47ab , 0x83 , 0x73 , 0x1c , 0x62 , 0x95 , 0xeb , 0x10 , 0xb3)] interface IDWriteInlineObject (IDWriteInlineObjectVtbl) : IUnknown (IUnknownVtbl) { fn Draw (clientDrawingContext : * mut c_void , renderer : * mut IDWriteTextRenderer , originX : FLOAT , originY : FLOAT , isSideways : BOOL , isRightToLeft : BOOL , clientDrawingEffect : * mut IUnknown ,) -> HRESULT , fn GetMetrics (metrics : * mut DWRITE_INLINE_OBJECT_METRICS ,) -> HRESULT , fn GetOverhangMetrics (overhangs : * mut DWRITE_OVERHANG_METRICS ,) -> HRESULT , fn GetBreakConditions (breakConditionBefore : * mut DWRITE_BREAK_CONDITION , breakConditionAfter : * mut DWRITE_BREAK_CONDITION ,) -> HRESULT , } }
};
}
