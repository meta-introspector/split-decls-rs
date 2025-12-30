// Generated macro for macro_3429 (macro)
macro_rules! Depcrate_shared_dxgi1_3macro_3429 {
() => {
// Module: crate::shared::dxgi1_3
// Provides: {"macro_3429"}
// Dependencies: {}
RIDL ! { # [uuid (0x2633066b , 0x4514 , 0x4c7a , 0x8f , 0xd8 , 0x12 , 0xea , 0x98 , 0x05 , 0x9d , 0x18)] interface IDXGIDecodeSwapChain (IDXGIDecodeSwapChainVtbl) : IUnknown (IUnknownVtbl) { fn PresentBuffer (BufferToPresent : UINT , SyncInterval : UINT , Flags : UINT ,) -> HRESULT , fn SetSourceRect (pRect : * const RECT ,) -> HRESULT , fn SetTargetRect (pRect : * const RECT ,) -> HRESULT , fn SetDestSize (Width : UINT , Height : UINT ,) -> HRESULT , fn GetSourceRect (pRect : * mut RECT ,) -> HRESULT , fn GetTargetRect (pRect : * mut RECT ,) -> HRESULT , fn GetDestSize (pWidth : * mut UINT , pHeight : * mut UINT ,) -> HRESULT , fn SetColorSpace (ColorSpace : DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS ,) -> HRESULT , fn GetColorSpace () -> DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS , } }
};
}
