// Generated macro for macro_26352 (macro)
macro_rules! Depcrate_um_d3d12macro_26352 {
() => {
// Module: crate::um::d3d12
// Provides: {"macro_26352"}
// Dependencies: {}
RIDL ! { # [uuid (0xc64226a8 , 0x9201 , 0x46af , 0xb4 , 0xcc , 0x53 , 0xfb , 0x9f , 0xf7 , 0x41 , 0x4f)] interface ID3D12PipelineLibrary (ID3D12PipelineLibraryVtbl) : ID3D12DeviceChild (ID3D12DeviceChildVtbl) { fn StorePipeline (pName : LPCWSTR , pPipeline : * mut ID3D12PipelineState ,) -> HRESULT , fn LoadGraphicsPipeline (pName : LPCWSTR , pDesc : * const D3D12_GRAPHICS_PIPELINE_STATE_DESC , riid : REFIID , ppPipelineState : * mut * mut c_void ,) -> HRESULT , fn LoadComputePipeline (pName : LPCWSTR , pDesc : * const D3D12_COMPUTE_PIPELINE_STATE_DESC , riid : REFIID , ppPipelineState : * mut * mut c_void ,) -> HRESULT , fn GetSerializedSize () -> SIZE_T , fn Serialize (pData : * mut c_void , DataSizeInBytes : SIZE_T ,) -> HRESULT , } }
};
}
