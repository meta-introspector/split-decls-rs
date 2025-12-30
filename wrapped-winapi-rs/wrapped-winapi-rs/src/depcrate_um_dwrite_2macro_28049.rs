// Generated macro for macro_28049 (macro)
macro_rules! Depcrate_um_dwrite_2macro_28049 {
() => {
// Module: crate::um::dwrite_2
// Provides: {"macro_28049"}
// Dependencies: {}
RIDL ! { # [uuid (0xfd882d06 , 0x8aba , 0x4fb8 , 0xb8 , 0x49 , 0x8b , 0xe8 , 0xb7 , 0x3e , 0x14 , 0xde)] interface IDWriteFontFallbackBuilder (IDWriteFontFallbackBuilderVtbl) : IUnknown (IUnknownVtbl) { fn AddMapping (ranges : * const DWRITE_UNICODE_RANGE , rangesCount : UINT32 , targetFamilyNames : * mut * const WCHAR , targetFamilyNamesCount : UINT32 , fontCollection : * mut IDWriteFontCollection , localeName : * const WCHAR , baseFamilyName : * const WCHAR , scale : FLOAT ,) -> HRESULT , fn AddMappings (fontFallback : * mut IDWriteFontFallback ,) -> HRESULT , fn CreateFontFallback (fontFallback : * mut * mut IDWriteFontFallback ,) -> HRESULT , } }
};
}
