// Generated macro for macro_24308 (macro)
macro_rules! Depcrate_um_d2d1svgmacro_24308 {
() => {
// Module: crate::um::d2d1svg
// Provides: {"macro_24308"}
// Dependencies: {}
RIDL ! { # [uuid (0xf1c0ca52 , 0x92a3 , 0x4f00 , 0xb4 , 0xce , 0xf3 , 0x56 , 0x91 , 0xef , 0xd9 , 0xd9)] interface ID2D1SvgStrokeDashArray (ID2D1SvgStrokeDashArrayVtbl) : ID2D1SvgAttribute (ID2D1SvgAttributeVtbl) { fn RemoveDashesAtEnd (dashesCount : UINT32 ,) -> HRESULT , fn UpdateDashes_1 (dashes : * const D2D1_SVG_LENGTH , dashesCount : UINT32 , startIndex : UINT32 ,) -> HRESULT , fn UpdateDashes_2 (dashes : * const FLOAT , dashesCount : UINT32 , startIndex : UINT32 ,) -> HRESULT , fn GetDashes_1 (dashes : * mut D2D1_SVG_LENGTH , dashesCount : UINT32 , startIndex : UINT32 ,) -> HRESULT , fn GetDashes_2 (dashes : * mut FLOAT , dashesCount : UINT32 , startIndex : UINT32 ,) -> HRESULT , fn GetDashesCount () -> UINT32 , } }
};
}
