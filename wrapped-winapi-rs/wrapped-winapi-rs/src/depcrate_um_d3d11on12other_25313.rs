// Generated macro for other_25313 (other)
macro_rules! Depcrate_um_d3d11on12other_25313 {
() => {
// Module: crate::um::d3d11on12
// Provides: {"other_25313"}
// Dependencies: {}
extern "system" { pub fn D3D11On12CreateDevice (pDevice : * mut IUnknown , Flags : UINT , pFeatureLevels : * const D3D_FEATURE_LEVEL , FeatureLevels : UINT , ppCommandQueues : * mut * mut IUnknown , NumQueues : UINT , NodeMask : UINT , ppDevice : * mut * mut ID3D11Device , ppImmediateContext : * mut * mut ID3D11DeviceContext , pChosenFeatureLevel : * mut D3D_FEATURE_LEVEL ,) -> HRESULT ; }
};
}
