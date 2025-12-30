// Generated macro for macro_28094 (macro)
macro_rules! Depcrate_um_dwrite_3macro_28094 {
() => {
// Module: crate::um::dwrite_3
// Provides: {"macro_28094"}
// Dependencies: {}
RIDL ! { # [uuid (0x07ddcd52 , 0x020e , 0x4de8 , 0xac , 0x33 , 0x6c , 0x95 , 0x3d , 0x83 , 0xf9 , 0x2d)] interface IDWriteTextLayout3 (IDWriteTextLayout3Vtbl) : IDWriteTextLayout2 (IDWriteTextLayout2Vtbl) { fn InvalidateLayout () -> HRESULT , fn SetLineSpacing (lineSpacingOptions : * const DWRITE_LINE_SPACING ,) -> HRESULT , fn GetLineSpacing (lineSpacingOptions : * mut DWRITE_LINE_SPACING ,) -> HRESULT , fn GetLineMetrics (lineMetrics : * mut DWRITE_LINE_METRICS1 , maxLineCount : UINT32 , actualLineCount : * mut UINT32 ,) -> HRESULT , } }
};
}
