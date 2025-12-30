// Generated macro for macro_25334 (macro)
macro_rules! Depcrate_um_d3d11sdklayersmacro_25334 {
() => {
// Module: crate::um::d3d11sdklayers
// Provides: {"macro_25334"}
// Dependencies: {}
RIDL ! { # [uuid (0x79cf2233 , 0x7536 , 0x4948 , 0x9d , 0x36 , 0x1e , 0x46 , 0x92 , 0xdc , 0x57 , 0x60)] interface ID3D11Debug (ID3D11DebugVtbl) : IUnknown (IUnknownVtbl) { fn SetFeatureMask (Mask : UINT ,) -> HRESULT , fn GetFeatureMask () -> UINT , fn SetPresentPerRenderOpDelay (Milliseconds : UINT ,) -> HRESULT , fn GetPresentPerRenderOpDelay () -> UINT , fn SetSwapChain (pSwapChain : * mut IDXGISwapChain ,) -> HRESULT , fn GetSwapChain (ppSwapChain : * mut * mut IDXGISwapChain ,) -> HRESULT , fn ValidateContext (pContext : * const ID3D11DeviceContext ,) -> HRESULT , fn ReportLiveDeviceObjects (Flags : D3D11_RLDO_FLAGS ,) -> HRESULT , fn ValidateContextForDispatch (pContext : * mut ID3D11DeviceContext ,) -> HRESULT , } }
};
}
