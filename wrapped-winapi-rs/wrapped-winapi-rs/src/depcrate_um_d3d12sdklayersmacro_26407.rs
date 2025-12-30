// Generated macro for macro_26407 (macro)
macro_rules! Depcrate_um_d3d12sdklayersmacro_26407 {
() => {
// Module: crate::um::d3d12sdklayers
// Provides: {"macro_26407"}
// Dependencies: {}
RIDL ! { # [uuid (0x3febd6dd , 0x4973 , 0x4787 , 0x81 , 0x94 , 0xe4 , 0x5f , 0x9e , 0x28 , 0x92 , 0x3e)] interface ID3D12DebugDevice1 (ID3D12DebugDevice1Vtbl) : IUnknown (IUnknownVtbl) { fn SetDebugParameter (Type : D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE , pData : * const c_void , DataSize : UINT ,) -> HRESULT , fn GetDebugParameter (Type : D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE , pData : * mut c_void , DataSize : UINT ,) -> HRESULT , fn ReportLiveDeviceObjects (Flags : D3D12_RLDO_FLAGS ,) -> HRESULT , } }
};
}
