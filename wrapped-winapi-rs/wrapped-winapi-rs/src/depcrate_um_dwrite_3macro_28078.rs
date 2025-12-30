// Generated macro for macro_28078 (macro)
macro_rules! Depcrate_um_dwrite_3macro_28078 {
() => {
// Module: crate::um::dwrite_3
// Provides: {"macro_28078"}
// Dependencies: {}
RIDL ! { # [uuid (0x53585141 , 0xd9f8 , 0x4095 , 0x83 , 0x21 , 0xd7 , 0x3c , 0xf6 , 0xbd , 0x11 , 0x6b)] interface IDWriteFontSet (IDWriteFontSetVtbl) : IUnknown (IUnknownVtbl) { fn GetFontCount () -> UINT32 , fn GetFontFaceReference (listIndex : UINT32 , fontFaceReference : * mut * mut IDWriteFontFaceReference ,) -> HRESULT , fn FindFontFaceReference (fontFaceReference : * mut IDWriteFontFaceReference , listIndex : * mut UINT32 , exists : * mut BOOL ,) -> HRESULT , fn FindFontFace (fontFace : * mut IDWriteFontFace , listIndex : * mut UINT32 , exists : * mut BOOL ,) -> HRESULT , fn GetPropertyValues_3 (propertyID : DWRITE_FONT_PROPERTY_ID , values : * mut * mut IDWriteStringList ,) -> HRESULT , fn GetPropertyValues_2 (propertyID : DWRITE_FONT_PROPERTY_ID , preferredLocaleNames : * const WCHAR , values : * mut * mut IDWriteStringList ,) -> HRESULT , fn GetPropertyValues_1 (listIndex : UINT32 , propertyId : DWRITE_FONT_PROPERTY_ID , exists : * mut BOOL , values : * mut * mut IDWriteLocalizedStrings ,) -> HRESULT , fn GetPropertyOccurrenceCount (property : * const DWRITE_FONT_PROPERTY , propertyOccurrenceCount : * mut UINT32 ,) -> HRESULT , fn GetMatchingFonts_2 (familyName : * const WCHAR , fontWeight : DWRITE_FONT_WEIGHT , fontStretch : DWRITE_FONT_STRETCH , fontStyle : DWRITE_FONT_STYLE , filteredSet : * mut * mut IDWriteFontSet ,) -> HRESULT , fn GetMatchingFonts_1 (properties : * const DWRITE_FONT_PROPERTY , propertyCount : UINT32 , filteredSet : * mut * mut IDWriteFontSet ,) -> HRESULT , } }
};
}
