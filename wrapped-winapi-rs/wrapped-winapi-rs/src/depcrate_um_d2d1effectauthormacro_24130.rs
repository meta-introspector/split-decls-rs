// Generated macro for macro_24130 (macro)
macro_rules! Depcrate_um_d2d1effectauthormacro_24130 {
() => {
// Module: crate::um::d2d1effectauthor
// Provides: {"macro_24130"}
// Dependencies: {}
RIDL ! { # [uuid (0xef1a287d , 0x342a , 0x4f76 , 0x8f , 0xdb , 0xda , 0x0d , 0x6e , 0xa9 , 0xf9 , 0x2b)] interface ID2D1Transform (ID2D1TransformVtbl) : ID2D1TransformNode (ID2D1TransformNodeVtbl) { fn MapOutputRectToInputRects (outputRect : * const D2D_RECT_L , inputRects : * mut D2D_RECT_L , inputRectsCount : UINT32 ,) -> HRESULT , fn MapInputRectsToOutputRect (inputRects : * const D2D_RECT_L , inputOpaqueSubRects : * const D2D_RECT_L , inputRectCount : UINT32 , outputRect : * mut D2D_RECT_L , outputOpaqueSubRect : * mut D2D_RECT_L ,) -> HRESULT , fn MapInvalidRect (inputIndex : UINT32 , invalidInputRect : D2D_RECT_L , invalidOutputRect : * mut D2D_RECT_L ,) -> HRESULT , } }
};
}
