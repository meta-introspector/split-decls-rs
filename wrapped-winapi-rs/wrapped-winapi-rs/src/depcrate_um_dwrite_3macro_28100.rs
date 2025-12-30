// Generated macro for macro_28100 (macro)
macro_rules! Depcrate_um_dwrite_3macro_28100 {
() => {
// Module: crate::um::dwrite_3
// Provides: {"macro_28100"}
// Dependencies: {}
RIDL ! { # [uuid (0x98eff3a5 , 0xb667 , 0x479a , 0xb1 , 0x45 , 0xe2 , 0xfa , 0x5b , 0x9f , 0xdc , 0x29)] interface IDWriteFontFace5 (IDWriteFontFace5Vtbl) : IDWriteFontFace4 (IDWriteFontFace4Vtbl) { fn GetFontAxisValueCount () -> UINT32 , fn GetFontAxisValues (values : * mut DWRITE_FONT_AXIS_VALUE , valueCount : UINT32 ,) -> HRESULT , fn HasVariations () -> BOOL , fn GetFontResource (resource : * mut * mut IDWriteFontResource ,) -> HRESULT , fn Equals (fontFace : * mut IDWriteFontFace ,) -> BOOL , } }
};
}
