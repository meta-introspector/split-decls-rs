// Generated macro for macro_23858 (macro)
macro_rules! Depcrate_um_d2d1macro_23858 {
() => {
// Module: crate::um::d2d1
// Provides: {"macro_23858"}
// Dependencies: {}
RIDL ! { # [uuid (0x2cd906ab , 0x12e2 , 0x11dc , 0x9f , 0xed , 0x00 , 0x11 , 0x43 , 0xa0 , 0x55 , 0xf9)] interface ID2D1LinearGradientBrush (ID2D1LinearGradientBrushVtbl) : ID2D1Brush (ID2D1BrushVtbl) { fn SetStartPoint (startPoint : D2D1_POINT_2F ,) -> () , fn SetEndPoint (endPoint : D2D1_POINT_2F ,) -> () , # [fixme] fn GetStartPoint () -> D2D1_POINT_2F , # [fixme] fn GetEndPoint () -> D2D1_POINT_2F , fn GetGradientStopCollection (gradientStopCollection : * mut * mut ID2D1GradientStopCollection ,) -> () , } }
};
}
