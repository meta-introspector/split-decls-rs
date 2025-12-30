// Generated macro for macro_28095 (macro)
macro_rules! Depcrate_um_dwrite_3macro_28095 {
() => {
// Module: crate::um::dwrite_3
// Provides: {"macro_28095"}
// Dependencies: {}
RIDL ! { # [uuid (0x27f2a904 , 0x4eb8 , 0x441d , 0x96 , 0x78 , 0x05 , 0x63 , 0xf5 , 0x3e , 0x3e , 0x2f)] interface IDWriteFontFace4 (IDWriteFontFace4Vtbl) : IDWriteFontFace3 (IDWriteFontFace3Vtbl) { fn GetGlyphImageFormats_2 (glyph : UINT16 , ppemFirst : UINT32 , ppemLast : UINT32 , formats : * mut DWRITE_GLYPH_IMAGE_FORMATS ,) -> HRESULT , fn GetGlyphImageFormats_1 () -> DWRITE_GLYPH_IMAGE_FORMATS , fn GetGlyphImageData (glyph : UINT16 , ppem : UINT32 , format : DWRITE_GLYPH_IMAGE_FORMATS , data : * mut DWRITE_GLYPH_IMAGE_DATA , context : * mut * mut c_void ,) -> HRESULT , fn ReleaseGlyphImageData (context : * mut c_void ,) -> () , } }
};
}
