// Generated macro for macro_26413 (macro)
macro_rules! Depcrate_um_d3d12sdklayersmacro_26413 {
() => {
// Module: crate::um::d3d12sdklayers
// Provides: {"macro_26413"}
// Dependencies: {}
RIDL ! { # [uuid (0x102ca951 , 0x311b , 0x4b01 , 0xb1 , 0x1f , 0xec , 0xb8 , 0x3e , 0x06 , 0x1b , 0x37)] interface ID3D12DebugCommandList1 (ID3D12DebugCommandList1Vtbl) : IUnknown (IUnknownVtbl) { fn AssertResourceState (pResource : * mut ID3D12Resource , Subresource : UINT , State : UINT ,) -> BOOL , fn SetDebugParameter (Type : D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE , pData : * const c_void , DataSize : UINT ,) -> HRESULT , fn GetDebugParameter (Type : D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE , pData : * mut c_void , DataSize : UINT ,) -> HRESULT , } }
};
}
