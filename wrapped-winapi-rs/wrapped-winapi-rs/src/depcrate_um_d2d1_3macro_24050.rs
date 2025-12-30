// Generated macro for macro_24050 (macro)
macro_rules! Depcrate_um_d2d1_3macro_24050 {
() => {
// Module: crate::um::d2d1_3
// Provides: {"macro_24050"}
// Dependencies: {}
RIDL ! { # [uuid (0x4dc583bf , 0x3a10 , 0x438a , 0x87 , 0x22 , 0xe9 , 0x76 , 0x52 , 0x24 , 0xf1 , 0xf1)] interface ID2D1SpriteBatch (ID2D1SpriteBatchVtbl) : ID2D1Resource (ID2D1ResourceVtbl) { fn AddSprites (spriteCount : UINT32 , destinationRectangle : * const D2D1_RECT_F , sourceRectangles : * const D2D1_RECT_U , colors : * const D2D1_COLOR_F , transforms : * const D2D1_MATRIX_3X2_F , destinationRectanglesStride : UINT32 , sourceRectanglesStride : UINT32 , colorsStride : UINT32 , transformsStride : D2D1_MATRIX_3X2_F ,) -> HRESULT , fn SetSprites (startIndex : UINT32 , spriteCount : UINT32 , destinationRectangle : * const D2D1_RECT_F , sourceRectangles : * const D2D1_RECT_U , colors : * const D2D1_COLOR_F , transforms : * const D2D1_MATRIX_3X2_F , destinationRectanglesStride : UINT32 , sourceRectanglesStride : UINT32 , colorsStride : UINT32 , transformsStride : D2D1_MATRIX_3X2_F ,) -> HRESULT , fn GetSprites (startIndex : UINT32 , spriteCount : UINT32 , destinationRectangle : * mut D2D1_RECT_F , sourceRectangles : * mut D2D1_RECT_U , colors : * mut D2D1_COLOR_F , transforms : * mut D2D1_MATRIX_3X2_F ,) -> HRESULT , fn GetSpriteCount () -> UINT32 , fn Clear () -> () , } }
};
}
