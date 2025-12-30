// Generated macro for macro_40455 (macro)
macro_rules! Depcrate_um_wincodecmacro_40455 {
() => {
// Module: crate::um::wincodec
// Provides: {"macro_40455"}
// Dependencies: {}
RIDL ! { # [uuid (0x3b16811b , 0x6a43 , 0x4ec9 , 0xa8 , 0x13 , 0x3d , 0x93 , 0x0c , 0x13 , 0xb9 , 0x40)] interface IWICBitmapFrameDecode (IWICBitmapFrameDecodeVtbl) : IWICBitmapSource (IWICBitmapSourceVtbl) { fn GetMetadataQueryReader (ppIMetadataQueryReader : * mut * mut IWICMetadataQueryReader ,) -> HRESULT , fn GetColorContexts (cCount : UINT , ppIColorContexts : * mut * mut IWICColorContext , pcActualCount : * mut UINT ,) -> HRESULT , fn GetThumbnail (ppIThumbnail : * mut * mut IWICBitmapSource ,) -> HRESULT , } }
};
}
