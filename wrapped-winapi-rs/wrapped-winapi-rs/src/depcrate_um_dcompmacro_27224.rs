// Generated macro for macro_27224 (macro)
macro_rules! Depcrate_um_dcompmacro_27224 {
() => {
// Module: crate::um::dcomp
// Provides: {"macro_27224"}
// Dependencies: {}
RIDL ! { # [uuid (0x0b74b9e8 , 0xcdd6 , 0x492f , 0xbb , 0xbc , 0x5e , 0xd3 , 0x21 , 0x57 , 0x02 , 0x6d)] interface IDCompositionAffineTransform2DEffect (IDCompositionAffineTransform2DEffectVtbl) : IDCompositionFilterEffect (IDCompositionFilterEffectVtbl) { fn SetInterpolationMode (interpolationMode : D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE ,) -> HRESULT , fn SetBorderMode (borderMode : D2D1_BORDER_MODE ,) -> HRESULT , fn SetTransformMatrix (transformMatrix : * const D2D1_MATRIX_3X2_F ,) -> HRESULT , fn SetTransformMatrixElement_2 (row : c_int , column : c_int , animation : * const IDCompositionAnimation ,) -> HRESULT , fn SetTransformMatrixElement_1 (row : c_int , column : c_int , value : c_float ,) -> HRESULT , fn SetSharpness_2 (animation : * const IDCompositionAnimation ,) -> HRESULT , fn SetSharpness_1 (sharpness : c_float ,) -> HRESULT , } }
};
}
