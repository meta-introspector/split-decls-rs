// Generated macro for macro_25315 (macro)
macro_rules! Depcrate_um_d3d11on12macro_25315 {
() => {
// Module: crate::um::d3d11on12
// Provides: {"macro_25315"}
// Dependencies: {}
RIDL ! { # [uuid (0x85611e73 , 0x70a9 , 0x490e , 0x96 , 0x14 , 0xa9 , 0xe3 , 0x02 , 0x77 , 0x79 , 0x04)] interface ID3D11On12Device (ID3D11On12DeviceVtbl) : IUnknown (IUnknownVtbl) { fn CreateWrappedResource (pResource12 : * mut IUnknown , pFlags11 : * const D3D11_RESOURCE_FLAGS , InState : D3D12_RESOURCE_STATES , OutState : D3D12_RESOURCE_STATES , riid : * const IID , ppResource11 : * mut * mut c_void ,) -> HRESULT , fn ReleaseWrappedResources (ppResources : * mut * mut ID3D11Resource , NumResources : UINT ,) -> () , fn AcquireWrappedResources (ppResources : * mut * mut ID3D11Resource , NumResources : UINT ,) -> () , } }
};
}
