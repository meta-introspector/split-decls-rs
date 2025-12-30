// Generated macro for macro_40454 (macro)
macro_rules! Depcrate_um_wincodecmacro_40454 {
() => {
// Module: crate::um::wincodec
// Provides: {"macro_40454"}
// Dependencies: {}
RIDL ! { # [uuid (0x3aff9cce , 0xbe95 , 0x4303 , 0xb9 , 0x27 , 0xe7 , 0xd1 , 0x6f , 0xf4 , 0xa6 , 0x13)] interface IWICPlanarBitmapSourceTransform (IWICPlanarBitmapSourceTransformVtbl) : IUnknown (IUnknownVtbl) { fn DoesSupportTransform (puiWidth : * mut UINT , puiHeight : * mut UINT , dstTransform : WICBitmapTransformOptions , dstPlanarOptions : WICPlanarOptions , pguidDstFormats : * const WICPixelFormatGUID , pPlaneDescriptions : * mut WICBitmapPlaneDescription , cPlanes : UINT , pfIsSupported : * mut BOOL ,) -> HRESULT , fn CopyPixels (prcSource : * const WICRect , uiWidth : UINT , uiHeight : UINT , dstTransform : WICBitmapTransformOptions , dstPlanarOptions : WICPlanarOptions , pDstPlanes : * const WICBitmapPlane , cPlanes : UINT ,) -> HRESULT , } }
};
}
