// Generated macro for macro_25095 (macro)
macro_rules! Depcrate_um_d3d11macro_25095 {
() => {
// Module: crate::um::d3d11
// Provides: {"macro_25095"}
// Dependencies: {}
RIDL ! { # [uuid (0x31627037 , 0x53ab , 0x4200 , 0x90 , 0x61 , 0x05 , 0xfa , 0xa9 , 0xab , 0x45 , 0xf9)] interface ID3D11VideoProcessorEnumerator (ID3D11VideoProcessorEnumeratorVtbl) : ID3D11DeviceChild (ID3D11DeviceChildVtbl) { fn GetVideoProcessorContentDesc (pContentDesc : * mut D3D11_VIDEO_PROCESSOR_CONTENT_DESC ,) -> HRESULT , fn CheckVideoProcessorFormat (Format : DXGI_FORMAT , pFlags : * mut UINT ,) -> HRESULT , fn GetVideoProcessorCaps (pCaps : * mut D3D11_VIDEO_PROCESSOR_CAPS ,) -> HRESULT , fn GetVideoProcessorRateConversionCaps (TypeIndex : UINT , pCaps : * mut D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS ,) -> HRESULT , fn GetVideoProcessorCustomRate (TypeIndex : UINT , CustomRateIndex : UINT , pRate : * mut D3D11_VIDEO_PROCESSOR_CUSTOM_RATE ,) -> HRESULT , fn GetVideoProcessorFilterRange (Filter : D3D11_VIDEO_PROCESSOR_FILTER , Range : * mut D3D11_VIDEO_PROCESSOR_FILTER_RANGE ,) -> HRESULT , } }
};
}
