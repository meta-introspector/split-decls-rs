// Generated macro for macro_28102 (macro)
macro_rules! Depcrate_um_dwrite_3macro_28102 {
() => {
// Module: crate::um::dwrite_3
// Provides: {"macro_28102"}
// Dependencies: {}
RIDL ! { # [uuid (0x1f803a76 , 0x6871 , 0x48e8 , 0x98 , 0x7f , 0xb9 , 0x75 , 0x55 , 0x1c , 0x50 , 0xf2)] interface IDWriteFontResource (IDWriteFontResourceVtbl) : IUnknown (IUnknownVtbl) { fn GetFontFile (fontFile : * mut * mut IDWriteFontFile ,) -> HRESULT , fn GetFontFaceIndex () -> UINT32 , fn GetFontAxisCount () -> UINT32 , fn GetDefaultFontAxisValues (values : * const DWRITE_FONT_AXIS_VALUE , numValues : UINT32 ,) -> HRESULT , fn GetFontAxisRanges (ranges : * const DWRITE_FONT_AXIS_RANGE , numRanges : UINT32 ,) -> HRESULT , fn GetFontAxisAttributes (axis : UINT32 ,) -> DWRITE_FONT_AXIS_ATTRIBUTES , fn GetAxisNames (axis : UINT32 , names : * mut * mut IDWriteLocalizedStrings ,) -> HRESULT , fn GetAxisValueNameCount (axis : UINT32 ,) -> UINT32 , fn GetAxisValueNames (axis : UINT32 , axisValue : UINT32 , axisRange : * mut DWRITE_FONT_AXIS_RANGE , names : * mut * mut IDWriteLocalizedStrings ,) -> HRESULT , fn HasVariations () -> BOOL , fn CreateFontFace (simulations : DWRITE_FONT_SIMULATIONS , axisValues : * const DWRITE_FONT_AXIS_VALUE , numValues : UINT32 , fontFace : * mut * mut IDWriteFontFace5 ,) -> HRESULT , fn CreateFontFaceReference (simulations : DWRITE_FONT_SIMULATIONS , axisValues : * const DWRITE_FONT_AXIS_VALUE , numValues : UINT32 , reference : * mut * mut IDWriteFontFaceReference1 ,) -> HRESULT , } }
};
}
