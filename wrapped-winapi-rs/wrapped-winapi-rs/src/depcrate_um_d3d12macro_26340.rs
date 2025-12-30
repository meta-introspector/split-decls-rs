// Generated macro for macro_26340 (macro)
macro_rules! Depcrate_um_d3d12macro_26340 {
() => {
// Module: crate::um::d3d12
// Provides: {"macro_26340"}
// Dependencies: {}
RIDL ! { # [uuid (0x696442be , 0xa72e , 0x4059 , 0xbc , 0x79 , 0x5b , 0x5c , 0x98 , 0x04 , 0x0f , 0xad)] interface ID3D12Resource (ID3D12ResourceVtbl) : ID3D12Pageable (ID3D12PageableVtbl) { fn Map (Subresource : UINT , pReadRange : * const D3D12_RANGE , ppData : * mut * mut c_void ,) -> HRESULT , fn Unmap (Subresource : UINT , pWrittenRange : * const D3D12_RANGE ,) -> () , # [fixme] fn GetDesc () -> D3D12_RESOURCE_DESC , fn GetGPUVirtualAddress () -> D3D12_GPU_VIRTUAL_ADDRESS , fn WriteToSubresource (DstSubresource : UINT , pDstBox : * const D3D12_BOX , pSrcData : * const c_void , SrcRowPitch : UINT , SrcDepthPitch : UINT ,) -> HRESULT , fn ReadFromSubresource (pDstData : * mut c_void , DstRowPitch : UINT , DstDepthPitch : UINT , SrcSubresource : UINT , pSrcBox : * const D3D12_BOX ,) -> HRESULT , fn GetHeapProperties (pHeapProperties : * mut D3D12_HEAP_PROPERTIES , pHeapFlags : * mut D3D12_HEAP_FLAGS ,) -> HRESULT , } }
};
}
