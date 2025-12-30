// Generated macro for macro_27919 (macro)
macro_rules! Depcrate_um_dwritemacro_27919 {
() => {
// Module: crate::um::dwrite
// Provides: {"macro_27919"}
// Dependencies: {}
RIDL ! { # [uuid (0xacd16696 , 0x8c14 , 0x4f5d , 0x87 , 0x7e , 0xfe , 0x3f , 0xc1 , 0xd3 , 0x27 , 0x37)] interface IDWriteFont (IDWriteFontVtbl) : IUnknown (IUnknownVtbl) { fn GetFontFamily (fontFamily : * mut * mut IDWriteFontFamily ,) -> HRESULT , fn GetWeight () -> DWRITE_FONT_WEIGHT , fn GetStretch () -> DWRITE_FONT_STRETCH , fn GetStyle () -> DWRITE_FONT_STYLE , fn IsSymbolFont () -> BOOL , fn GetFaceNames (names : * mut * mut IDWriteLocalizedStrings ,) -> HRESULT , fn GetInformationalStrings (informationalStringId : DWRITE_INFORMATIONAL_STRING_ID , informationalStrings : * mut * mut IDWriteLocalizedStrings , exists : * mut BOOL ,) -> HRESULT , fn GetSimulations () -> DWRITE_FONT_SIMULATIONS , fn GetMetrics (fontMetrics : * mut DWRITE_FONT_METRICS ,) -> () , fn HasCharacter (unicodeValue : UINT32 , exists : * mut BOOL ,) -> HRESULT , fn CreateFontFace (fontFace : * mut * mut IDWriteFontFace ,) -> HRESULT , } }
};
}
