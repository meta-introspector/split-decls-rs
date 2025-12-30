// Generated macro for macro_3338 (macro)
macro_rules! Depcrate_shared_dxgimacro_3338 {
() => {
// Module: crate::shared::dxgi
// Provides: {"macro_3338"}
// Dependencies: {}
RIDL ! { # [uuid (0x310d36a0 , 0xd2e7 , 0x4c0a , 0xaa , 0x04 , 0x6a , 0x9d , 0x23 , 0xb8 , 0x88 , 0x6a)] interface IDXGISwapChain (IDXGISwapChainVtbl) : IDXGIDeviceSubObject (IDXGIDeviceSubObjectVtbl) { fn Present (SyncInterval : UINT , Flags : UINT ,) -> HRESULT , fn GetBuffer (Buffer : UINT , riid : REFIID , ppSurface : * mut * mut c_void ,) -> HRESULT , fn SetFullscreenState (Fullscreen : BOOL , pTarget : * mut IDXGIOutput ,) -> HRESULT , fn GetFullscreenState (pFullscreen : * mut BOOL , ppTarget : * mut * mut IDXGIOutput ,) -> HRESULT , fn GetDesc (pDesc : * mut DXGI_SWAP_CHAIN_DESC ,) -> HRESULT , fn ResizeBuffers (BufferCount : UINT , Width : UINT , Height : UINT , NewFormat : DXGI_FORMAT , SwapChainFlags : UINT ,) -> HRESULT , fn ResizeTarget (pNewTargetParameters : * const DXGI_MODE_DESC ,) -> HRESULT , fn GetContainingOutput (ppOutput : * mut * mut IDXGIOutput ,) -> HRESULT , fn GetFrameStatistics (pStats : * mut DXGI_FRAME_STATISTICS ,) -> HRESULT , fn GetLastPresentCount (pLastPresentCount : * mut UINT ,) -> HRESULT , } }
};
}
