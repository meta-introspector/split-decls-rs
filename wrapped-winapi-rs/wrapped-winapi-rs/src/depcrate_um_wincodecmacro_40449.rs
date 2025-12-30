// Generated macro for macro_40449 (macro)
macro_rules! Depcrate_um_wincodecmacro_40449 {
() => {
// Module: crate::um::wincodec
// Provides: {"macro_40449"}
// Dependencies: {}
RIDL ! { # [uuid (0x00000105 , 0xa8f2 , 0x4877 , 0xba , 0x0a , 0xfd , 0x2b , 0x66 , 0x45 , 0xfb , 0x94)] interface IWICBitmapFrameEncode (IWICBitmapFrameEncodeVtbl) : IUnknown (IUnknownVtbl) { fn Initialize (pIEncoderOptions : * const IPropertyBag2 ,) -> HRESULT , fn SetSize (uiWidth : UINT , uiHeight : UINT ,) -> HRESULT , fn SetResolution (dpiX : c_double , dpiY : c_double ,) -> HRESULT , fn SetPixelFormat (pPixelFormat : * mut WICPixelFormatGUID ,) -> HRESULT , fn SetColorContexts (cCount : UINT , ppIColorContext : * const * const IWICColorContext ,) -> HRESULT , fn SetPalette (pIPalette : * const IWICPalette ,) -> HRESULT , fn SetThumbnail (pIThumbnail : * const IWICBitmapSource ,) -> HRESULT , fn WritePixels (lineCount : UINT , cbStride : UINT , cbBufferSize : UINT , pbPixels : * const BYTE ,) -> HRESULT , fn WriteSource (pIBitmapSource : * const IWICBitmapSource , prc : * const WICRect ,) -> HRESULT , fn Commit () -> HRESULT , fn GetMetadataQueryWriter (ppIMetadataQueryWriter : * mut * mut IWICMetadataQueryWriter ,) -> HRESULT , } }
};
}
