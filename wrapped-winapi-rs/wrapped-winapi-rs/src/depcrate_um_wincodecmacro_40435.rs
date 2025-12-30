// Generated macro for macro_40435 (macro)
macro_rules! Depcrate_um_wincodecmacro_40435 {
() => {
// Module: crate::um::wincodec
// Provides: {"macro_40435"}
// Dependencies: {}
RIDL ! { # [uuid (0xbebee9cb , 0x83b0 , 0x4dcc , 0x81 , 0x32 , 0xb0 , 0xaa , 0xa5 , 0x5e , 0xac , 0x96)] interface IWICPlanarFormatConverter (IWICPlanarFormatConverterVtbl) : IWICBitmapSource (IWICBitmapSourceVtbl) { fn Initialize (ppPlanes : * const * const IWICBitmapSource , cPlanes : UINT , dstFormat : REFWICPixelFormatGUID , dither : WICBitmapDitherType , pIPalette : * const IWICPalette , alphaThresholdPercent : c_double , paletteTranslate : WICBitmapPaletteType ,) -> HRESULT , fn CanConvert (pSrcPixelFormats : * const WICPixelFormatGUID , cSrcPlanes : UINT , dstPixelFormat : REFWICPixelFormatGUID , pfCanConvert : * mut BOOL ,) -> HRESULT , } }
};
}
