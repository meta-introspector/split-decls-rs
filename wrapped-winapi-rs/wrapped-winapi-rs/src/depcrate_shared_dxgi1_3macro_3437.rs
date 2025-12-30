// Generated macro for macro_3437 (macro)
macro_rules! Depcrate_shared_dxgi1_3macro_3437 {
() => {
// Module: crate::shared::dxgi1_3
// Provides: {"macro_3437"}
// Dependencies: {}
RIDL ! { # [uuid (0xdd95b90b , 0xf05f , 0x4f6a , 0xbd , 0x65 , 0x25 , 0xbf , 0xb2 , 0x64 , 0xbd , 0x84)] interface IDXGISwapChainMedia (IDXGISwapChainMediaVtbl) : IUnknown (IUnknownVtbl) { fn GetFrameStatisticsMedia (pStats : * mut DXGI_FRAME_STATISTICS_MEDIA ,) -> HRESULT , fn SetPresentDuration (Duration : UINT ,) -> HRESULT , fn CheckPresentDurationSupport (DesiredPresentDuration : UINT , pClosestSmallerPresentDuration : * mut UINT , pClosestLargerPresentDuration : * mut UINT ,) -> HRESULT , } }
};
}
