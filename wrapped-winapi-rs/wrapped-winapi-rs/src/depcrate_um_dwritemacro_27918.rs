// Generated macro for macro_27918 (macro)
macro_rules! Depcrate_um_dwritemacro_27918 {
() => {
// Module: crate::um::dwrite
// Provides: {"macro_27918"}
// Dependencies: {}
RIDL ! { # [uuid (0xda20d8ef , 0x812a , 0x4c43 , 0x98 , 0x02 , 0x62 , 0xec , 0x4a , 0xbd , 0x7a , 0xdd)] interface IDWriteFontFamily (IDWriteFontFamilyVtbl) : IDWriteFontList (IDWriteFontListVtbl) { fn GetFamilyNames (names : * mut * mut IDWriteLocalizedStrings ,) -> HRESULT , fn GetFirstMatchingFont (weight : DWRITE_FONT_WEIGHT , stretch : DWRITE_FONT_STRETCH , style : DWRITE_FONT_STYLE , matchingFont : * mut * mut IDWriteFont ,) -> HRESULT , fn GetMatchingFonts (weight : DWRITE_FONT_WEIGHT , stretch : DWRITE_FONT_STRETCH , style : DWRITE_FONT_STYLE , matchingFonts : * mut * mut IDWriteFontList ,) -> HRESULT , } }
};
}
