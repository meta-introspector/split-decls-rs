// Generated macro for macro_28023 (macro)
macro_rules! Depcrate_um_dwrite_1macro_28023 {
() => {
// Module: crate::um::dwrite_1
// Provides: {"macro_28023"}
// Dependencies: {}
RIDL ! { # [uuid (0xacd16696 , 0x8c14 , 0x4f5d , 0x87 , 0x7e , 0xfe , 0x3f , 0xc1 , 0xd3 , 0x27 , 0x38)] interface IDWriteFont1 (IDWriteFont1Vtbl) : IDWriteFont (IDWriteFontVtbl) { fn GetMetrics (fontMetrics : * mut DWRITE_FONT_METRICS1 ,) -> () , fn GetPanose (panose : * mut DWRITE_PANOSE ,) -> () , fn GetUnicodeRanges (maxRangeCount : UINT32 , unicodeRanges : * mut DWRITE_UNICODE_RANGE , actualRangeCount : * mut UINT32 ,) -> HRESULT , fn IsMonospacedFont () -> BOOL , } }
};
}
