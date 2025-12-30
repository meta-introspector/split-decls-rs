// Generated macro for macro_27916 (macro)
macro_rules! Depcrate_um_dwritemacro_27916 {
() => {
// Module: crate::um::dwrite
// Provides: {"macro_27916"}
// Dependencies: {}
RIDL ! { # [uuid (0xa84cee02 , 0x3eea , 0x4eee , 0xa8 , 0x27 , 0x87 , 0xc1 , 0xa0 , 0x2a , 0x0f , 0xcc)] interface IDWriteFontCollection (IDWriteFontCollectionVtbl) : IUnknown (IUnknownVtbl) { fn GetFontFamilyCount () -> UINT32 , fn GetFontFamily (index : UINT32 , fontFamily : * mut * mut IDWriteFontFamily ,) -> HRESULT , fn FindFamilyName (familyName : * const WCHAR , index : * mut UINT32 , exists : * mut BOOL ,) -> HRESULT , fn GetFontFromFontFace (fontFace : * mut IDWriteFontFace , font : * mut * mut IDWriteFont ,) -> HRESULT , } }
};
}
