// Generated macro for macro_40434 (macro)
macro_rules! Depcrate_um_wincodecmacro_40434 {
() => {
// Module: crate::um::wincodec
// Provides: {"macro_40434"}
// Dependencies: {}
RIDL ! { # [uuid (0x00000301 , 0xa8f2 , 0x4877 , 0xba , 0x0a , 0xfd , 0x2b , 0x66 , 0x45 , 0xfb , 0x94)] interface IWICFormatConverter (IWICFormatConverterVtbl) : IWICBitmapSource (IWICBitmapSourceVtbl) { fn Initialize (pISource : * const IWICBitmapSource , dstFormat : REFWICPixelFormatGUID , dither : WICBitmapDitherType , pIPalette : * const IWICPalette , alphaThresholdPercent : c_double , paletteTranslate : WICBitmapPaletteType ,) -> HRESULT , fn CanConvert (srcPixelFormat : REFWICPixelFormatGUID , dstPixelFormat : REFWICPixelFormatGUID , pfCanConvert : * mut BOOL ,) -> HRESULT , } }
};
}
