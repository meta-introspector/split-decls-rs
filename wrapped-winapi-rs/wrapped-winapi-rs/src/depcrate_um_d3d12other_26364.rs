// Generated macro for other_26364 (other)
macro_rules! Depcrate_um_d3d12other_26364 {
() => {
// Module: crate::um::d3d12
// Provides: {"other_26364"}
// Dependencies: {}
extern "system" { pub fn D3D12GetDebugInterface (riid : REFGUID , ppvDebug : * mut * mut c_void ,) -> HRESULT ; pub fn D3D12EnableExperimentalFeatures (NumFeatures : UINT , pIIDs : * const IID , pConfigurationStructs : * mut c_void , pConfigurationStructSizes : * mut UINT ,) -> HRESULT ; }
};
}
