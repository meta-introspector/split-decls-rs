// Generated macro for macro_27945 (macro)
macro_rules! Depcrate_um_dwritemacro_27945 {
() => {
// Module: crate::um::dwrite
// Provides: {"macro_27945"}
// Dependencies: {}
RIDL ! { # [uuid (0x688e1a58 , 0x5094 , 0x47c8 , 0xad , 0xc8 , 0xfb , 0xce , 0xa6 , 0x0a , 0xe9 , 0x2b)] interface IDWriteTextAnalysisSource (IDWriteTextAnalysisSourceVtbl) : IUnknown (IUnknownVtbl) { fn GetTextAtPosition (textPosition : UINT32 , textString : * mut * const WCHAR , textLength : * mut UINT32 ,) -> HRESULT , fn GetTextBeforePosition (textPosition : UINT32 , textString : * mut * const WCHAR , textLength : * mut UINT32 ,) -> HRESULT , fn GetParagraphReadingDirection () -> DWRITE_READING_DIRECTION , fn GetLocaleName (textPosition : UINT32 , textLength : * mut UINT32 , localeName : * mut * const WCHAR ,) -> HRESULT , fn GetNumberSubstitution (textPosition : UINT32 , textLength : * mut UINT32 , numberSubstitution : * mut * mut IDWriteNumberSubstitution ,) -> HRESULT , } }
};
}
