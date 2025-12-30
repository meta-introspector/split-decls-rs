// Generated macro for macro_3465 (macro)
macro_rules! Depcrate_shared_dxgi1_4macro_3465 {
() => {
// Module: crate::shared::dxgi1_4
// Provides: {"macro_3465"}
// Dependencies: {}
RIDL ! { # [uuid (0x94d99bdb , 0xf1f8 , 0x4ab0 , 0xb2 , 0x36 , 0x7d , 0xa0 , 0x17 , 0x0e , 0xda , 0xb1)] interface IDXGISwapChain3 (IDXGISwapChain3Vtbl) : IDXGISwapChain2 (IDXGISwapChain2Vtbl) { fn GetCurrentBackBufferIndex () -> UINT , fn CheckColorSpaceSupport (ColorSpace : DXGI_COLOR_SPACE_TYPE , pColorSpaceSupport : * mut UINT ,) -> HRESULT , fn SetColorSpace1 (ColorSpace : DXGI_COLOR_SPACE_TYPE ,) -> HRESULT , fn ResizeBuffers1 (BufferCount : UINT , Width : UINT , Height : UINT , Format : DXGI_FORMAT , SwapChainFlags : UINT , pCreationNodeMask : * const UINT , ppPresentQueue : * mut * mut IUnknown ,) -> HRESULT , } }
};
}
