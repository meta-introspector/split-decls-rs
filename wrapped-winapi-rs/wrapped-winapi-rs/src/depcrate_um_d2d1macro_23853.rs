// Generated macro for macro_23853 (macro)
macro_rules! Depcrate_um_d2d1macro_23853 {
() => {
// Module: crate::um::d2d1
// Provides: {"macro_23853"}
// Dependencies: {}
RIDL ! { # [uuid (0xa2296057 , 0xea42 , 0x4099 , 0x98 , 0x3b , 0x53 , 0x9f , 0xb6 , 0x50 , 0x54 , 0x26)] interface ID2D1Bitmap (ID2D1BitmapVtbl) : ID2D1Image (ID2D1ImageVtbl) { # [fixme] fn GetSize () -> D2D1_SIZE_F , # [fixme] fn GetPixelSize () -> D2D1_SIZE_U , # [fixme] fn GetPixelFormat () -> D2D1_PIXEL_FORMAT , fn GetDpi (dpiX : * mut FLOAT , dpiY : * mut FLOAT ,) -> () , fn CopyFromBitmap (destPoint : * const D2D1_POINT_2U , bitmap : * mut ID2D1Bitmap , srcRect : * const D2D1_RECT_U ,) -> HRESULT , fn CopyFromRenderTarget (destPoint : * const D2D1_POINT_2U , renderTarget : * mut ID2D1RenderTarget , srcRect : * const D2D1_RECT_U ,) -> HRESULT , fn CopyFromMemory (dstRect : * const D2D1_RECT_U , srcData : * const c_void , pitch : UINT32 ,) -> HRESULT , } }
};
}
