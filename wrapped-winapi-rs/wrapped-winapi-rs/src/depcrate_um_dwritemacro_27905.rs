// Generated macro for macro_27905 (macro)
macro_rules! Depcrate_um_dwritemacro_27905 {
() => {
// Module: crate::um::dwrite
// Provides: {"macro_27905"}
// Dependencies: {}
RIDL ! { # [uuid (0x739d886a , 0xcef5 , 0x47dc , 0x87 , 0x69 , 0x1a , 0x8b , 0x41 , 0xbe , 0xbb , 0xb0)] interface IDWriteFontFile (IDWriteFontFileVtbl) : IUnknown (IUnknownVtbl) { fn GetReferenceKey (fontFileReferenceKey : * mut * const c_void , fontFileReferenceKeySize : * mut UINT32 ,) -> HRESULT , fn GetLoader (fontFileLoader : * mut * mut IDWriteFontFileLoader ,) -> HRESULT , fn Analyze (isSupportedFontType : * mut BOOL , fontFileType : * mut DWRITE_FONT_FILE_TYPE , fontFaceType : * mut DWRITE_FONT_FACE_TYPE , numberOfFaces : * mut UINT32 ,) -> HRESULT , } }
};
}
