// Generated macro for macro_23867 (macro)
macro_rules! Depcrate_um_d2d1macro_23867 {
() => {
// Module: crate::um::d2d1
// Provides: {"macro_23867"}
// Dependencies: {}
RIDL ! { # [uuid (0x2cd9069e , 0x12e2 , 0x11dc , 0x9f , 0xed , 0x00 , 0x11 , 0x43 , 0xa0 , 0x55 , 0xf9)] interface ID2D1SimplifiedGeometrySink (ID2D1SimplifiedGeometrySinkVtbl) : IUnknown (IUnknownVtbl) { fn SetFillMode (fillMode : D2D1_FILL_MODE ,) -> () , fn SetSegmentFlags (vertexFlags : D2D1_PATH_SEGMENT ,) -> () , fn BeginFigure (startPoint : D2D1_POINT_2F , figureBegin : D2D1_FIGURE_BEGIN ,) -> () , fn AddLines (points : * const D2D1_POINT_2F , pointsCount : UINT32 ,) -> () , fn AddBeziers (beziers : * const D2D1_BEZIER_SEGMENT , beziersCount : UINT32 ,) -> () , fn EndFigure (figureEnd : D2D1_FIGURE_END ,) -> () , fn Close () -> HRESULT , } }
};
}
