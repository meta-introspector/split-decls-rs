// Generated macro for macro_40464 (macro)
macro_rules! Depcrate_um_wincodecmacro_40464 {
() => {
// Module: crate::um::wincodec
// Provides: {"macro_40464"}
// Dependencies: {}
RIDL ! { # [uuid (0xd8cd007f , 0xd08f , 0x4191 , 0x9b , 0xfc , 0x23 , 0x6e , 0xa7 , 0xf0 , 0xe4 , 0xb5)] interface IWICBitmapDecoderInfo (IWICBitmapDecoderInfoVtbl) : IWICBitmapCodecInfo (IWICBitmapCodecInfoVtbl) { fn GetPatterns (cbSizePatterns : UINT , pPatterns : * mut WICBitmapPattern , pcPatterns : * mut UINT , pcbPatternsActual : * mut UINT ,) -> HRESULT , fn MatchesPattern (pIStream : * const IStream , pfMatches : * mut BOOL ,) -> HRESULT , fn CreateInstance (ppIBitmapDecoder : * mut * mut IWICBitmapDecoder ,) -> HRESULT , } }
};
}
