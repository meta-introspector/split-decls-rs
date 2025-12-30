// Generated macro for macro_40453 (macro)
macro_rules! Depcrate_um_wincodecmacro_40453 {
() => {
// Module: crate::um::wincodec
// Provides: {"macro_40453"}
// Dependencies: {}
RIDL ! { # [uuid (0x3b16811b , 0x6a43 , 0x4ec9 , 0xb7 , 0x13 , 0x3d , 0x5a , 0x0c , 0x13 , 0xb9 , 0x40)] interface IWICBitmapSourceTransform (IWICBitmapSourceTransformVtbl) : IUnknown (IUnknownVtbl) { fn CopyPixels (prc : * const WICRect , uiWidth : UINT , uiHeight : UINT , pguidDstFormat : * const WICPixelFormatGUID , dstTransform : WICBitmapTransformOptions , nStride : UINT , cbBufferSize : UINT , pbBuffer : * mut BYTE ,) -> HRESULT , fn GetClosestSize (puiWidth : * mut UINT , puiHeight : * mut UINT ,) -> HRESULT , fn GetClosestPixelFormat (pguidDstFormat : * mut WICPixelFormatGUID ,) -> HRESULT , fn DoesSupportTransform (dstTransform : WICBitmapTransformOptions , pfIsSupported : * mut BOOL ,) -> HRESULT , } }
};
}
