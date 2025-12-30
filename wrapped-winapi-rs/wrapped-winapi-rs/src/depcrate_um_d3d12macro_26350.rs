// Generated macro for macro_26350 (macro)
macro_rules! Depcrate_um_d3d12macro_26350 {
() => {
// Module: crate::um::d3d12
// Provides: {"macro_26350"}
// Dependencies: {}
RIDL ! { # [uuid (0x0ec870a6 , 0x5d7e , 0x4c22 , 0x8c , 0xfc , 0x5b , 0xaa , 0xe0 , 0x76 , 0x16 , 0xed)] interface ID3D12CommandQueue (ID3D12CommandQueueVtbl) : ID3D12Pageable (ID3D12PageableVtbl) { fn UpdateTileMappings (pResource : * mut ID3D12Resource , NumResourceRegions : UINT , pResourceRegionStartCoordinates : * const D3D12_TILED_RESOURCE_COORDINATE , pResourceRegionSizes : * const D3D12_TILE_REGION_SIZE , pHeap : * mut ID3D12Heap , NumRanges : UINT , pRangeFlags : * const D3D12_TILE_RANGE_FLAGS , pHeapRangeStartOffsets : * const UINT , pRangeTileCounts : * const UINT , Flags : D3D12_TILE_MAPPING_FLAGS ,) -> () , fn CopyTileMappings (pDstResource : * mut ID3D12Resource , pDstRegionStartCoordinate : * const D3D12_TILED_RESOURCE_COORDINATE , pSrcResource : * mut ID3D12Resource , pSrcRegionStartCoordinate : * const D3D12_TILED_RESOURCE_COORDINATE , pRegionSize : * const D3D12_TILE_REGION_SIZE , Flags : D3D12_TILE_MAPPING_FLAGS ,) -> () , fn ExecuteCommandLists (NumCommandLists : UINT , ppCommandLists : * const * mut ID3D12CommandList ,) -> () , fn SetMarker (Metadata : UINT , pData : * const c_void , Size : UINT ,) -> () , fn BeginEvent (Metadata : UINT , pData : * const c_void , Size : UINT ,) -> () , fn EndEvent () -> () , fn Signal (pFence : * mut ID3D12Fence , Value : UINT64 ,) -> HRESULT , fn Wait (pFence : * mut ID3D12Fence , Value : UINT64 ,) -> HRESULT , fn GetTimestampFrequency (pFrequency : * mut UINT64 ,) -> HRESULT , fn GetClockCalibration (pGpuTimestamp : * mut UINT64 , pCpuTimestamp : * mut UINT64 ,) -> HRESULT , # [fixme] fn GetDesc () -> D3D12_COMMAND_QUEUE_DESC , } }
};
}
