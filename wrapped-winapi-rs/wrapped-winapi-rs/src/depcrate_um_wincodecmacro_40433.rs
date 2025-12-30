// Generated macro for macro_40433 (macro)
macro_rules! Depcrate_um_wincodecmacro_40433 {
() => {
// Module: crate::um::wincodec
// Provides: {"macro_40433"}
// Dependencies: {}
RIDL ! { # [uuid (0x00000120 , 0xa8f2 , 0x4877 , 0xba , 0x0a , 0xfd , 0x2b , 0x66 , 0x45 , 0xfb , 0x94)] interface IWICBitmapSource (IWICBitmapSourceVtbl) : IUnknown (IUnknownVtbl) { fn GetSize (puiWidth : * mut UINT , puiHeight : * mut UINT ,) -> HRESULT , fn GetPixelFormat (pPixelFormat : * mut WICPixelFormatGUID ,) -> HRESULT , fn GetResolution (pDpiX : * mut c_double , pDpiY : * mut c_double ,) -> HRESULT , fn CopyPalette (pIPalette : * mut IWICPalette ,) -> HRESULT , fn CopyPixels (prc : * const WICRect , cbStride : UINT , cbBufferSize : UINT , pbBuffer : * mut BYTE ,) -> HRESULT , } }
};
}
