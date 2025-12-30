// Generated macro for macro_40448 (macro)
macro_rules! Depcrate_um_wincodecmacro_40448 {
() => {
// Module: crate::um::wincodec
// Provides: {"macro_40448"}
// Dependencies: {}
RIDL ! { # [uuid (0x00000103 , 0xa8f2 , 0x4877 , 0xba , 0x0a , 0xfd , 0x2b , 0x66 , 0x45 , 0xfb , 0x94)] interface IWICBitmapEncoder (IWICBitmapEncoderVtbl) : IUnknown (IUnknownVtbl) { fn Initialize (pIStream : * const IStream , cacheOption : WICBitmapEncoderCacheOption ,) -> HRESULT , fn GetContainerFormat (pguidContainerFormat : * mut GUID ,) -> HRESULT , fn GetEncoderInfo (ppIEncoderInfo : * mut * mut IWICBitmapEncoderInfo ,) -> HRESULT , fn SetColorContexts (cCount : UINT , ppIColorContext : * const * const IWICColorContext ,) -> HRESULT , fn SetPalette (pIPalette : * const IWICPalette ,) -> HRESULT , fn SetThumbnail (pIThumbnail : * const IWICBitmapSource ,) -> HRESULT , fn SetPreview (pIPreview : * const IWICBitmapSource ,) -> HRESULT , fn CreateNewFrame (ppIFrameEncode : * mut * mut IWICBitmapFrameEncode , ppIEncoderOptions : * mut * mut IPropertyBag2 ,) -> HRESULT , fn Commit () -> HRESULT , fn GetMetadataQueryWriter (ppIMetadataQueryWriter : * mut * mut IWICMetadataQueryWriter ,) -> HRESULT , } }
};
}
