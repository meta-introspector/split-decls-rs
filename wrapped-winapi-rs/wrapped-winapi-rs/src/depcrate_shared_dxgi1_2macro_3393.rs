// Generated macro for macro_3393 (macro)
macro_rules! Depcrate_shared_dxgi1_2macro_3393 {
() => {
// Module: crate::shared::dxgi1_2
// Provides: {"macro_3393"}
// Dependencies: {}
RIDL ! { # [uuid (0x50c83a1c , 0xe072 , 0x4c48 , 0x87 , 0xb0 , 0x36 , 0x30 , 0xfa , 0x36 , 0xa6 , 0xd0)] interface IDXGIFactory2 (IDXGIFactory2Vtbl) : IDXGIFactory1 (IDXGIFactory1Vtbl) { fn IsWindowedStereoEnabled () -> BOOL , fn CreateSwapChainForHwnd (pDevice : * mut IUnknown , hWnd : HWND , pDesc : * const DXGI_SWAP_CHAIN_DESC1 , pFullscreenDesc : * const DXGI_SWAP_CHAIN_FULLSCREEN_DESC , pRestrictToOutput : * mut IDXGIOutput , ppSwapChain : * mut * mut IDXGISwapChain1 ,) -> HRESULT , fn CreateSwapChainForCoreWindow (pDevice : * mut IUnknown , pWindow : * mut IUnknown , pDesc : * const DXGI_SWAP_CHAIN_DESC1 , pRestrictToOutput : * mut IDXGIOutput , ppSwapChain : * mut * mut IDXGISwapChain1 ,) -> HRESULT , fn GetSharedResourceAdapterLuid (hResource : HANDLE , pLuid : * mut LUID ,) -> HRESULT , fn RegisterStereoStatusWindow (WindowHandle : HWND , wMsg : UINT , pdwCookie : * mut DWORD ,) -> HRESULT , fn RegisterStereoStatusEvent (hEvent : HANDLE , pdwCookie : * mut DWORD ,) -> HRESULT , fn UnregisterStereoStatus (dwCookie : DWORD ,) -> () , fn RegisterOcclusionStatusWindow (WindowHandle : HWND , wMsg : UINT , pdwCookie : * mut DWORD ,) -> HRESULT , fn RegisterOcclusionStatusEvent (hEvent : HANDLE , pdwCookie : * mut DWORD ,) -> HRESULT , fn UnregisterOcclusionStatus (dwCookie : DWORD ,) -> () , fn CreateSwapChainForComposition (pDevice : * mut IUnknown , pDesc : * const DXGI_SWAP_CHAIN_DESC1 , pRestrictToOutput : * mut IDXGIOutput , ppSwapChain : * mut * mut IDXGISwapChain1 ,) -> HRESULT , } }
};
}
