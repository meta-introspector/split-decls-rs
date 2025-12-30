// Generated macro for macro_3433 (macro)
macro_rules! Depcrate_shared_dxgi1_3macro_3433 {
() => {
// Module: crate::shared::dxgi1_3
// Provides: {"macro_3433"}
// Dependencies: {}
RIDL ! { # [uuid (0x41e7d1f2 , 0xa591 , 0x4f7b , 0xa2 , 0xe5 , 0xfa , 0x9c , 0x84 , 0x3e , 0x1c , 0x12)] interface IDXGIFactoryMedia (IDXGIFactoryMediaVtbl) : IUnknown (IUnknownVtbl) { fn CreateSwapChainForCompositionSurfaceHandle (pDevice : * mut IUnknown , hSurface : HANDLE , pDesc : * const DXGI_SWAP_CHAIN_DESC1 , pRestrictToOutput : * mut IDXGIOutput , ppSwapChain : * mut * mut IDXGISwapChain1 ,) -> HRESULT , fn CreateDecodeSwapChainForCompositionSurfaceHandle (pDevice : * mut IUnknown , hSurface : HANDLE , pDesc : * mut DXGI_DECODE_SWAP_CHAIN_DESC , pYuvDecodeBuffers : * mut IDXGIResource , pRestrictToOutput : * mut IDXGIOutput , ppSwapChain : * mut * mut IDXGIDecodeSwapChain ,) -> HRESULT , } }
};
}
