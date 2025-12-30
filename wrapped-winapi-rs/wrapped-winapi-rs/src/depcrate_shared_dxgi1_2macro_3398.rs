// Generated macro for macro_3398 (macro)
macro_rules! Depcrate_shared_dxgi1_2macro_3398 {
() => {
// Module: crate::shared::dxgi1_2
// Provides: {"macro_3398"}
// Dependencies: {}
RIDL ! { # [uuid (0x790a45f7 , 0x0d42 , 0x4876 , 0x98 , 0x3a , 0x0a , 0x55 , 0xcf , 0xe6 , 0xf4 , 0xaa)] interface IDXGISwapChain1 (IDXGISwapChain1Vtbl) : IDXGISwapChain (IDXGISwapChainVtbl) { fn GetDesc1 (pDesc : * mut DXGI_SWAP_CHAIN_DESC1 ,) -> HRESULT , fn GetFullscreenDesc (pDesc : * mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC ,) -> HRESULT , fn GetHwnd (pHwnd : * mut HWND ,) -> HRESULT , fn GetCoreWindow (refiid : REFGUID , ppUnk : * mut * mut c_void ,) -> HRESULT , fn Present1 (SyncInterval : UINT , PresentFlags : UINT , pPresentParameters : * const DXGI_PRESENT_PARAMETERS ,) -> HRESULT , fn IsTemporaryMonoSupported () -> BOOL , fn GetRestrictToOutput (ppRestrictToOutput : * mut * mut IDXGIOutput ,) -> HRESULT , fn SetBackgroundColor (pColor : * const DXGI_RGBA ,) -> HRESULT , fn GetBackgroundColor (pColor : * mut DXGI_RGBA ,) -> HRESULT , fn SetRotation (Rotation : DXGI_MODE_ROTATION ,) -> HRESULT , fn GetRotation (pRotation : * mut DXGI_MODE_ROTATION ,) -> HRESULT , } }
};
}
