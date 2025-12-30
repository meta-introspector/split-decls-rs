// Generated macro for macro_28084 (macro)
macro_rules! Depcrate_um_dwrite_3macro_28084 {
() => {
// Module: crate::um::dwrite_3
// Provides: {"macro_28084"}
// Dependencies: {}
RIDL ! { # [uuid (0x29748ed6 , 0x8c9c , 0x4a6a , 0xbe , 0x0b , 0xd9 , 0x12 , 0xe8 , 0x53 , 0x89 , 0x44)] interface IDWriteFont3 (IDWriteFont3Vtbl) : IDWriteFont2 (IDWriteFont2Vtbl) { fn CreateFontFace (fontFace : * mut * mut IDWriteFontFace3 ,) -> HRESULT , fn Equals (font : * mut IDWriteFont ,) -> BOOL , fn GetFontFaceReference (fontFaceReference : * mut * mut IDWriteFontFaceReference ,) -> HRESULT , fn HasCharacter (unicodeValue : UINT32 ,) -> BOOL , fn GetLocality () -> DWRITE_LOCALITY , } }
};
}
