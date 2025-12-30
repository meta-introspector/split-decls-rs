// Generated macro for macro_23859 (macro)
macro_rules! Depcrate_um_d2d1macro_23859 {
() => {
// Module: crate::um::d2d1
// Provides: {"macro_23859"}
// Dependencies: {}
RIDL ! { # [uuid (0x2cd906ac , 0x12e2 , 0x11dc , 0x9f , 0xed , 0x00 , 0x11 , 0x43 , 0xa0 , 0x55 , 0xf9)] interface ID2D1RadialGradientBrush (ID2D1RadialGradientBrushVtbl) : ID2D1Brush (ID2D1BrushVtbl) { fn SetCenter (center : D2D1_POINT_2F ,) -> () , fn SetGradientOriginOffset (gradientOriginOffset : D2D1_POINT_2F ,) -> () , fn SetRadiusX (radiusX : FLOAT ,) -> () , fn SetRadiusY (radiusY : FLOAT ,) -> () , # [fixme] fn GetCenter () -> D2D1_POINT_2F , # [fixme] fn GetGradientOriginOffset () -> D2D1_POINT_2F , fn GetRadiusX () -> FLOAT , fn GetRadiusY () -> FLOAT , fn GetGradientStopCollection (gradientStopCollection : * mut * mut ID2D1GradientStopCollection ,) -> () , } }
};
}
