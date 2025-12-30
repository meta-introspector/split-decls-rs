// Generated macro for macro_24309 (macro)
macro_rules! Depcrate_um_d2d1svgmacro_24309 {
() => {
// Module: crate::um::d2d1svg
// Provides: {"macro_24309"}
// Dependencies: {}
RIDL ! { # [uuid (0x9dbe4c0d , 0x3572 , 0x4dd9 , 0x98 , 0x25 , 0x55 , 0x30 , 0x81 , 0x3b , 0xb7 , 0x12)] interface ID2D1SvgPointCollection (ID2D1SvgPointCollectionVtbl) : ID2D1SvgAttribute (ID2D1SvgAttributeVtbl) { fn RemovePointsAtEnd (pointsCount : UINT32 ,) -> HRESULT , fn UpdatePoints (points : * const D2D1_POINT_2F , pointsCount : UINT32 , startIndex : UINT32 ,) -> HRESULT , fn GetPoints (points : * mut D2D1_POINT_2F , pointsCount : UINT32 , startIndex : UINT32 ,) -> HRESULT , fn GetPointsCount () -> UINT32 , } }
};
}
