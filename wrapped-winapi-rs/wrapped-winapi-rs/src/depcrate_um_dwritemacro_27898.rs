// Generated macro for macro_27898 (macro)
macro_rules! Depcrate_um_dwritemacro_27898 {
() => {
// Module: crate::um::dwrite
// Provides: {"macro_27898"}
// Dependencies: {}
RIDL ! { # [uuid (0xb2d9f3ec , 0xc9fe , 0x4a11 , 0xa2 , 0xec , 0xd8 , 0x62 , 0x08 , 0xf7 , 0xc0 , 0xa2)] interface IDWriteLocalFontFileLoader (IDWriteLocalFontFileLoaderVtbl) : IDWriteFontFileLoader (IDWriteFontFileLoaderVtbl) { fn GetFilePathLengthFromKey (fontFileReferenceKey : * const c_void , fontFileReferenceKeySize : UINT32 , filePathLength : * mut UINT32 ,) -> HRESULT , fn GetFilePathFromKey (fontFileReferenceKey : * const c_void , fontFileReferenceKeySize : UINT32 , filePath : * mut WCHAR , filePathSize : UINT32 ,) -> HRESULT , fn GetLastWriteTimeFromKey (fontFileReferenceKey : * const c_void , fontFileReferenceKeySize : UINT32 , lastWriteTime : * mut FILETIME ,) -> HRESULT , } }
};
}
