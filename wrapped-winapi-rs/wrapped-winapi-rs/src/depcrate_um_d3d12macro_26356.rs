// Generated macro for macro_26356 (macro)
macro_rules! Depcrate_um_d3d12macro_26356 {
() => {
// Module: crate::um::d3d12
// Provides: {"macro_26356"}
// Dependencies: {}
RIDL ! { # [uuid (0x77acce80 , 0x638e , 0x4e65 , 0x88 , 0x95 , 0xc1 , 0xf2 , 0x33 , 0x86 , 0x86 , 0x3e)] interface ID3D12Device1 (ID3D12Device1Vtbl) : ID3D12Device (ID3D12DeviceVtbl) { fn CreatePipelineLibrary (pLibraryBlob : * const c_void , BlobLength : SIZE_T , riid : REFIID , ppPipelineLibrary : * mut * mut c_void ,) -> HRESULT , fn SetEventOnMultipleFenceCompletion (ppFences : * const * mut ID3D12Fence , pFenceValues : * const UINT64 , NumFences : UINT , Flags : D3D12_MULTIPLE_FENCE_WAIT_FLAGS , hEvent : HANDLE ,) -> HRESULT , fn SetResidencyPriority (NumObjects : UINT , ppObjects : * const * mut ID3D12Pageable , pPriorities : * const D3D12_RESIDENCY_PRIORITY ,) -> HRESULT , } }
};
}
