// Generated macro for macro_26349 (macro)
macro_rules! Depcrate_um_d3d12macro_26349 {
() => {
// Module: crate::um::d3d12
// Provides: {"macro_26349"}
// Dependencies: {}
RIDL ! { # [uuid (0x553103fb , 0x1fe7 , 0x4557 , 0xbb , 0x38 , 0x94 , 0x6d , 0x7d , 0x0e , 0x7c , 0xa7)] interface ID3D12GraphicsCommandList1 (ID3D12GraphicsCommandList1Vtbl) : ID3D12GraphicsCommandList (ID3D12GraphicsCommandListVtbl) { fn AtomicCopyBufferUINT (pDstBuffer : * mut ID3D12Resource , DstOffset : UINT64 , pSrcBuffer : * mut ID3D12Resource , SrcOffset : UINT64 , Dependencies : UINT , ppDependentResources : * const * mut ID3D12Resource , pDependentSubresourceRanges : * mut D3D12_SUBRESOURCE_RANGE_UINT64 ,) -> () , fn AtomicCopyBufferUINT64 (pDstBuffer : * mut ID3D12Resource , DstOffset : UINT64 , pSrcBuffer : * mut ID3D12Resource , SrcOffset : UINT64 , Dependencies : UINT , ppDependentResources : * const * mut ID3D12Resource , pDependentSubresourceRanges : * mut D3D12_SUBRESOURCE_RANGE_UINT64 ,) -> () , fn OMSetDepthBounds (Min : FLOAT , Max : FLOAT ,) -> () , fn SetSamplePositions (NumSamplesPerPixel : UINT , NumPixels : UINT , pSamplePositions : * mut D3D12_SAMPLE_POSITION ,) -> () , fn ResolveSubresourceRegion (pDstResource : * mut ID3D12Resource , DstSubresource : UINT , DstX : UINT , DstY : UINT , pSrcResource : * mut ID3D12Resource , SrcSubresource : UINT , pSrcRect : * mut D3D12_RECT , Format : DXGI_FORMAT , ResolveMode : D3D12_RESOLVE_MODE ,) -> () , } }
};
}
