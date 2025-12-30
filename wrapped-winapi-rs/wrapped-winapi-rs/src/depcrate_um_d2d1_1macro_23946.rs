// Generated macro for macro_23946 (macro)
macro_rules! Depcrate_um_d2d1_1macro_23946 {
() => {
// Module: crate::um::d2d1_1
// Provides: {"macro_23946"}
// Dependencies: {}
RIDL ! { # [uuid (0x28211a43 , 0x7d89 , 0x476f , 0x81 , 0x81 , 0x2d , 0x61 , 0x59 , 0xb2 , 0x20 , 0xad)] interface ID2D1Effect (ID2D1EffectVtbl) : ID2D1Properties (ID2D1PropertiesVtbl) { fn SetInput (index : UINT32 , input : * const ID2D1Image , invalidate : BOOL ,) -> () , fn SetInputCount (inputCount : UINT32 ,) -> HRESULT , fn GetInput (index : UINT32 , input : * mut * mut ID2D1Image ,) -> () , fn GetInputCount () -> UINT32 , fn GetOutput (outputImage : * mut * mut ID2D1Image ,) -> () , } }
};
}
