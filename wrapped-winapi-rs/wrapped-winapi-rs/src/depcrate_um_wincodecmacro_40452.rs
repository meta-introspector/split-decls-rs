// Generated macro for macro_40452 (macro)
macro_rules! Depcrate_um_wincodecmacro_40452 {
() => {
// Module: crate::um::wincodec
// Provides: {"macro_40452"}
// Dependencies: {}
RIDL ! { # [uuid (0x9edde9e7 , 0x8dee , 0x47ea , 0x99 , 0xdf , 0xe6 , 0xfa , 0xf2 , 0xed , 0x44 , 0xbf)] interface IWICBitmapDecoder (IWICBitmapDecoderVtbl) : IUnknown (IUnknownVtbl) { fn QueryCapability (pIStream : * const IStream , pdwCapability : * mut DWORD ,) -> HRESULT , fn Initialize (pIStream : * const IStream , cacheOptions : WICDecodeOptions ,) -> HRESULT , fn GetContainerFormat (pguidContainerFormat : * mut GUID ,) -> HRESULT , fn GetDecoderInfo (ppIDecoderInfo : * mut * mut IWICBitmapDecoderInfo ,) -> HRESULT , fn CopyPalette (pIPalette : * const IWICPalette ,) -> HRESULT , fn GetMetadataQueryReader (ppIMetadataQueryReader : * mut * mut IWICMetadataQueryReader ,) -> HRESULT , fn GetPreview (ppIBitmapSource : * mut * mut IWICBitmapSource ,) -> HRESULT , fn GetColorContexts (cCount : UINT , ppIColorContexts : * mut * mut IWICColorContext , pcActualCount : * mut UINT ,) -> HRESULT , fn GetThumbnail (ppIThumbnail : * mut * mut IWICBitmapSource ,) -> HRESULT , fn GetFrameCount (pCount : * mut UINT ,) -> HRESULT , fn GetFrame (index : UINT , ppIBitmapFrame : * mut * mut IWICBitmapFrameDecode ,) -> HRESULT , } }
};
}
