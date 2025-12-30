// Generated macro for macro_23868 (macro)
macro_rules! Depcrate_um_d2d1macro_23868 {
() => {
// Module: crate::um::d2d1
// Provides: {"macro_23868"}
// Dependencies: {}
RIDL ! { # [uuid (0x2cd9069f , 0x12e2 , 0x11dc , 0x9f , 0xed , 0x00 , 0x11 , 0x43 , 0xa0 , 0x55 , 0xf9)] interface ID2D1GeometrySink (ID2D1GeometrySinkVtbl) : ID2D1SimplifiedGeometrySink (ID2D1SimplifiedGeometrySinkVtbl) { fn AddLine (point : D2D1_POINT_2F ,) -> () , fn AddBezier (bezier : * const D2D1_BEZIER_SEGMENT ,) -> () , fn AddQuadraticBezier (bezier : * const D2D1_QUADRATIC_BEZIER_SEGMENT ,) -> () , fn AddQuadraticBeziers (beziers : * const D2D1_QUADRATIC_BEZIER_SEGMENT , beziersCount : UINT32 ,) -> () , fn AddArc (arc : * const D2D1_ARC_SEGMENT ,) -> () , } }
};
}
