// Generated macro for macro_27214 (macro)
macro_rules! Depcrate_um_dcompmacro_27214 {
() => {
// Module: crate::um::dcomp
// Provides: {"macro_27214"}
// Dependencies: {}
RIDL ! { # [uuid (0xc1170a22 , 0x3ce2 , 0x4966 , 0x90 , 0xd4 , 0x55 , 0x40 , 0x8b , 0xfc , 0x84 , 0xc4)] interface IDCompositionColorMatrixEffect (IDCompositionColorMatrixEffectVtbl) : IDCompositionFilterEffect (IDCompositionFilterEffectVtbl) { fn SetMatrix (matrix : * const D2D1_MATRIX_5X4_F ,) -> HRESULT , fn SetMatrixElement_2 (row : c_int , column : c_int , animation : * const IDCompositionAnimation ,) -> HRESULT , fn SetMatrixElement_1 (row : c_int , column : c_int , value : c_float ,) -> HRESULT , fn SetAlphaMode (mode : D2D1_COLORMATRIX_ALPHA_MODE ,) -> HRESULT , fn SetClampOutput (clamp : BOOL ,) -> HRESULT , } }
};
}
