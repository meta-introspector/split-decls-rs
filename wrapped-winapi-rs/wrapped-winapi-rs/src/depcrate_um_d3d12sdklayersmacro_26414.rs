// Generated macro for macro_26414 (macro)
macro_rules! Depcrate_um_d3d12sdklayersmacro_26414 {
() => {
// Module: crate::um::d3d12sdklayers
// Provides: {"macro_26414"}
// Dependencies: {}
RIDL ! { # [uuid (0x09e0bf36 , 0x54ac , 0x484f , 0x88 , 0x47 , 0x4b , 0xae , 0xea , 0xb6 , 0x05 , 0x3f)] interface ID3D12DebugCommandList (ID3D12DebugCommandListVtbl) : IUnknown (IUnknownVtbl) { fn AssertResourceState (pResource : * mut ID3D12Resource , Subresource : UINT , State : UINT ,) -> BOOL , fn SetFeatureMask (Mask : D3D12_DEBUG_FEATURE ,) -> HRESULT , fn GetFeatureMask () -> D3D12_DEBUG_FEATURE , } }
};
}
