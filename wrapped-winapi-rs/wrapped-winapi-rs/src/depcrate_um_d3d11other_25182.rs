// Generated macro for other_25182 (other)
macro_rules! Depcrate_um_d3d11other_25182 {
() => {
// Module: crate::um::d3d11
// Provides: {"other_25182"}
// Dependencies: {}
extern "system" { pub fn D3D11CreateDevice (pAdapter : * mut IDXGIAdapter , DriverType : D3D_DRIVER_TYPE , Software : HMODULE , Flags : UINT , pFeatureLevels : * const D3D_FEATURE_LEVEL , FeatureLevels : UINT , SDKVersion : UINT , ppDevice : * mut * mut ID3D11Device , pFeatureLevel : * mut D3D_FEATURE_LEVEL , ppImmediateContext : * mut * mut ID3D11DeviceContext ,) -> HRESULT ; pub fn D3D11CreateDeviceAndSwapChain (pAdapter : * mut IDXGIAdapter , DriverType : D3D_DRIVER_TYPE , Software : HMODULE , Flags : UINT , pFeatureLevels : * const D3D_FEATURE_LEVEL , FeatureLevels : UINT , SDKVersion : UINT , pSwapChainDesc : * const DXGI_SWAP_CHAIN_DESC , ppSwapChain : * mut * mut IDXGISwapChain , ppDevice : * mut * mut ID3D11Device , pFeatureLevel : * mut D3D_FEATURE_LEVEL , ppImmediateContext : * mut * mut ID3D11DeviceContext ,) -> HRESULT ; }
};
}
