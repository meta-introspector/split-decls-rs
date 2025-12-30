// Generated macro for macro_24055 (macro)
macro_rules! Depcrate_um_d2d1_3macro_24055 {
() => {
// Module: crate::um::d2d1_3
// Provides: {"macro_24055"}
// Dependencies: {}
RIDL ! { # [uuid (0xaf671749 , 0xd241 , 0x4db8 , 0x8e , 0x41 , 0xdc , 0xc2 , 0xe5 , 0xc1 , 0xa4 , 0x38)] interface ID2D1SvgGlyphStyle (ID2D1SvgGlyphStyleVtbl) : ID2D1Resource (ID2D1ResourceVtbl) { fn SetFill (brush : * mut ID2D1Brush ,) -> HRESULT , fn GetFill (brush : * mut * mut ID2D1Brush ,) -> () , fn SetStroke (brush : * mut ID2D1Brush , strokeWidth : FLOAT , dashes : * const FLOAT , dashesCount : UINT32 , dashOffset : FLOAT ,) -> HRESULT , fn GetStrokeDashesCount () -> UINT32 , fn GetStroke (brush : * mut * mut ID2D1Brush , strokeWidth : * mut FLOAT , dashes : * mut FLOAT , dashesCount : UINT32 , dashOffset : * mut FLOAT ,) -> () , } }
};
}
