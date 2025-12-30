// Generated macro for macro_40513 (macro)
macro_rules! Depcrate_um_wincodecmacro_40513 {
() => {
// Module: crate::um::wincodec
// Provides: {"macro_40513"}
// Dependencies: {}
RIDL ! { # [uuid (0x5cacdb4c , 0x407e , 0x41b3 , 0xb9 , 0x36 , 0xd0 , 0xf0 , 0x10 , 0xcd , 0x67 , 0x32)] interface IWICDdsEncoder (IWICDdsEncoderVtbl) : IUnknown (IUnknownVtbl) { fn SetParameters (pParameters : * const WICDdsParameters ,) -> HRESULT , fn GetParameters (pParameters : * mut WICDdsParameters ,) -> HRESULT , fn CreateNewFrame (ppIFrameEncode : * mut * mut IWICBitmapFrameEncode , pArrayIndex : * mut UINT , pMipLevel : * mut UINT , pSliceIndex : * mut UINT ,) -> HRESULT , } }
};
}
