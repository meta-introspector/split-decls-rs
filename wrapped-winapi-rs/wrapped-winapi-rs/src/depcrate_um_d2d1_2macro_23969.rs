// Generated macro for macro_23969 (macro)
macro_rules! Depcrate_um_d2d1_2macro_23969 {
() => {
// Module: crate::um::d2d1_2
// Provides: {"macro_23969"}
// Dependencies: {}
RIDL ! { # [uuid (0xd37f57e4 , 0x6908 , 0x459f , 0xa1 , 0x99 , 0xe7 , 0x2f , 0x24 , 0xf7 , 0x99 , 0x87)] interface ID2D1DeviceContext1 (ID2D1DeviceContext1Vtbl) : ID2D1DeviceContext (ID2D1DeviceContextVtbl) { fn CreateFilledGeometryRealization (geometry : * mut ID2D1Geometry , flatteningTolerance : FLOAT , geometryRealization : * mut * mut ID2D1GeometryRealization ,) -> HRESULT , fn CreateStrokedGeometryRealization (geometry : * mut ID2D1Geometry , flatteningTolerance : FLOAT , strokeWidth : FLOAT , strokeStyle : * mut ID2D1StrokeStyle , geometryRealization : * mut * mut ID2D1GeometryRealization ,) -> HRESULT , fn DrawGeometryRealization (geometryRealization : * mut ID2D1GeometryRealization , brush : * mut ID2D1Brush ,) -> () , } }
};
}
