// Generated macro for macro_28083 (macro)
macro_rules! Depcrate_um_dwrite_3macro_28083 {
() => {
// Module: crate::um::dwrite_3
// Provides: {"macro_28083"}
// Dependencies: {}
RIDL ! { # [uuid (0x5e7fa7ca , 0xdde3 , 0x424c , 0x89 , 0xf0 , 0x9f , 0xcd , 0x6f , 0xed , 0x58 , 0xcd)] interface IDWriteFontFaceReference (IDWriteFontFaceReferenceVtbl) : IUnknown (IUnknownVtbl) { fn CreateFontFace (fontFace : * mut * mut IDWriteFontFace3 ,) -> HRESULT , fn CreateFontFaceWithSimulations (fontFaceSimulationFlags : DWRITE_FONT_SIMULATIONS , fontFace : * mut * mut IDWriteFontFace3 ,) -> HRESULT , fn Equals (fontFaceReference : * mut IDWriteFontFaceReference ,) -> BOOL , fn GetFontFaceIndex () -> UINT32 , fn GetSimulations () -> DWRITE_FONT_SIMULATIONS , fn GetFontFile (fontFile : * mut * mut IDWriteFontFile ,) -> HRESULT , fn GetLocalFileSize () -> UINT64 , fn GetFileSize () -> UINT64 , fn GetFileTime (lastWriteTime : * mut FILETIME ,) -> HRESULT , fn GetLocality () -> DWRITE_LOCALITY , fn EnqueueFontDownloadRequest () -> HRESULT , fn EnqueueCharacterDownloadRequest (characters : * const WCHAR , characterCount : UINT32 ,) -> HRESULT , fn EnqueueGlyphDownloadRequest (glyphIndices : * const UINT16 , glyphCount : UINT32 ,) -> HRESULT , fn EnqueueFileFragmentDownloadRequest (fileOffset : UINT64 , fragmentSize : UINT64 ,) -> HRESULT , } }
};
}
