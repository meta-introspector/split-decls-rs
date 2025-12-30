// Generated macro for other_23880 (other)
macro_rules! Depcrate_um_d2d1other_23880 {
() => {
// Module: crate::um::d2d1
// Provides: {"other_23880"}
// Dependencies: {}
extern "system" { pub fn D2D1CreateFactory (factoryType : D2D1_FACTORY_TYPE , riid : REFIID , pFactoryOptions : * const D2D1_FACTORY_OPTIONS , ppIFactory : * mut * mut c_void ,) -> HRESULT ; pub fn D2D1MakeRotateMatrix (angle : FLOAT , center : D2D1_POINT_2F , matrix : * mut D2D1_MATRIX_3X2_F ,) ; pub fn D2D1MakeSkewMatrix (angleX : FLOAT , angleY : FLOAT , center : D2D1_POINT_2F , matrix : * mut D2D1_MATRIX_3X2_F ,) ; pub fn D2D1IsMatrixInvertible (matrix : * const D2D1_MATRIX_3X2_F ,) -> BOOL ; pub fn D2D1InvertMatrix (matrix : * mut D2D1_MATRIX_3X2_F ,) -> BOOL ; pub fn D2D1ComputeMaximumScaleFactor (matrix : * const D2D1_MATRIX_3X2_F ,) -> FLOAT ; }
};
}
