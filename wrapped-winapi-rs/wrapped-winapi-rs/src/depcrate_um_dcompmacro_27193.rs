// Generated macro for macro_27193 (macro)
macro_rules! Depcrate_um_dcompmacro_27193 {
() => {
// Module: crate::um::dcomp
// Provides: {"macro_27193"}
// Dependencies: {}
RIDL ! { # [uuid (0x16cdff07 , 0xc503 , 0x419c , 0x83 , 0xf2 , 0x09 , 0x65 , 0xc7 , 0xaf , 0x1f , 0xa6)] interface IDCompositionMatrixTransform (IDCompositionMatrixTransformVtbl) : IDCompositionTransform (IDCompositionTransformVtbl) { fn SetMatrix (matrix : * const D2D_MATRIX_3X2_F ,) -> HRESULT , fn SetMatrixElement_2 (row : c_int , column : c_int , animation : * const IDCompositionAnimation ,) -> HRESULT , fn SetMatrixElement_1 (row : c_int , column : c_int , value : c_float ,) -> HRESULT , } }
};
}
