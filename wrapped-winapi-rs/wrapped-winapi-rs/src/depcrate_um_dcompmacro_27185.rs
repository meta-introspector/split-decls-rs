// Generated macro for macro_27185 (macro)
macro_rules! Depcrate_um_dcompmacro_27185 {
() => {
// Module: crate::um::dcomp
// Provides: {"macro_27185"}
// Dependencies: {}
RIDL ! { # [uuid (0x4d93059d , 0x097b , 0x4651 , 0x9a , 0x60 , 0xf0 , 0xf2 , 0x51 , 0x16 , 0xe2 , 0xf3)] interface IDCompositionVisual (IDCompositionVisualVtbl) : IUnknown (IUnknownVtbl) { fn SetOffsetX_2 (animation : * const IDCompositionAnimation ,) -> HRESULT , fn SetOffsetX_1 (offsetX : c_float ,) -> HRESULT , fn SetOffsetY_2 (animation : * const IDCompositionAnimation ,) -> HRESULT , fn SetOffsetY_1 (offsetY : c_float ,) -> HRESULT , fn SetTransform_2 (transform : * const IDCompositionTransform ,) -> HRESULT , fn SetTransform_1 (matrix : * const D2D_MATRIX_3X2_F ,) -> HRESULT , fn SetTransformParent (visual : * const IDCompositionVisual ,) -> HRESULT , fn SetEffect (effect : * const IDCompositionEffect ,) -> HRESULT , fn SetBitmapInterpolationMode (interpolationMode : DCOMPOSITION_BITMAP_INTERPOLATION_MODE ,) -> HRESULT , fn SetBorderMode (borderMode : DCOMPOSITION_BORDER_MODE ,) -> HRESULT , fn SetClip_2 (clip : * const IDCompositionClip ,) -> HRESULT , fn SetClip_1 (rect : * const D2D_RECT_F ,) -> HRESULT , fn SetContent (content : * const IUnknown ,) -> HRESULT , fn AddVisual (visual : * const IDCompositionVisual , insertAbove : BOOL , referenceVisual : * const IDCompositionVisual ,) -> HRESULT , fn RemoveVisual (visual : * const IDCompositionVisual ,) -> HRESULT , fn RemoveAllVisuals () -> HRESULT , fn SetCompositeMode (compositeMode : DCOMPOSITION_COMPOSITE_MODE ,) -> HRESULT , } }
};
}
