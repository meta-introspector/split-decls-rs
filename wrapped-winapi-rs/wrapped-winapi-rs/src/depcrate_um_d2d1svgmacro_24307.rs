// Generated macro for macro_24307 (macro)
macro_rules! Depcrate_um_d2d1svgmacro_24307 {
() => {
// Module: crate::um::d2d1svg
// Provides: {"macro_24307"}
// Dependencies: {}
RIDL ! { # [uuid (0xd59bab0a , 0x68a2 , 0x455b , 0xa5 , 0xdc , 0x9e , 0xb2 , 0x85 , 0x4e , 0x24 , 0x90)] interface ID2D1SvgPaint (ID2D1SvgPaintVtbl) : ID2D1SvgAttribute (ID2D1SvgAttributeVtbl) { fn SetPaintType (paintType : D2D1_SVG_PAINT_TYPE ,) -> HRESULT , fn GetPaintType () -> D2D1_SVG_PAINT_TYPE , fn SetColor (color : D2D1_COLOR_F ,) -> HRESULT , fn GetColor (color : * mut D2D1_COLOR_F ,) -> () , fn SetId (id : PCWSTR ,) -> HRESULT , fn GetId (id : PWSTR , idCount : UINT32 ,) -> HRESULT , fn GetIdLength () -> UINT32 , } }
};
}
