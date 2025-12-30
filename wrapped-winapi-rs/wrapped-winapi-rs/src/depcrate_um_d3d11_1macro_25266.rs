// Generated macro for macro_25266 (macro)
macro_rules! Depcrate_um_d3d11_1macro_25266 {
() => {
// Module: crate::um::d3d11_1
// Provides: {"macro_25266"}
// Dependencies: {}
RIDL ! { # [uuid (0x29da1d51 , 0x1321 , 0x4454 , 0x80 , 0x4b , 0xf5 , 0xfc , 0x9f , 0x86 , 0x1f , 0x0f)] interface ID3D11VideoDevice1 (ID3D11VideoDevice1Vtbl) : ID3D11VideoDevice (ID3D11VideoDeviceVtbl) { fn GetCryptoSessionPrivateDataSize (pCryptoType : * const GUID , pDecoderProfile : * const GUID , pKeyExchangeType : * const GUID , pPrivateInputSize : * mut UINT , pPrivateOutputSize : * mut UINT ,) -> HRESULT , fn GetVideoDecoderCaps (pDecoderProfile : * const GUID , SampleWidth : UINT , SampleHeight : UINT , pFrameRate : * const DXGI_RATIONAL , BitRate : UINT , pCryptoType : * const GUID , pDecoderCaps : * mut UINT ,) -> HRESULT , fn CheckVideoDecoderDownsampling (pInputDesc : * const D3D11_VIDEO_DECODER_DESC , InputColorSpace : DXGI_COLOR_SPACE_TYPE , pInputConfig : * const D3D11_VIDEO_DECODER_CONFIG , pFrameRate : * const DXGI_RATIONAL , pOutputDesc : * const D3D11_VIDEO_SAMPLE_DESC , pSupported : * mut BOOL , pRealTimeHint : * mut BOOL ,) -> HRESULT , fn RecommendVideoDecoderDownsampleParameters (pInputDesc : * const D3D11_VIDEO_DECODER_DESC , InputColorSpace : DXGI_COLOR_SPACE_TYPE , pInputConfig : * const D3D11_VIDEO_DECODER_CONFIG , pRecommendedOutputDesc : * mut D3D11_VIDEO_SAMPLE_DESC ,) -> HRESULT , } }
};
}
