// Generated macro for macro_3391 (macro)
macro_rules! Depcrate_shared_dxgi1_2macro_3391 {
() => {
// Module: crate::shared::dxgi1_2
// Provides: {"macro_3391"}
// Dependencies: {}
RIDL ! { # [uuid (0x05008617 , 0xfbfd , 0x4051 , 0xa7 , 0x90 , 0x14 , 0x48 , 0x84 , 0xb4 , 0xf6 , 0xa9)] interface IDXGIDevice2 (IDXGIDevice2Vtbl) : IDXGIDevice1 (IDXGIDevice1Vtbl) { fn OfferResources (NumResources : UINT , ppResources : * mut * mut IDXGIResource , Priority : DXGI_OFFER_RESOURCE_PRIORITY ,) -> HRESULT , fn ReclaimResources (NumResources : UINT , ppResources : * mut * mut IDXGIResource , pDiscarded : * mut BOOL ,) -> HRESULT , fn EnqueueSetEvent (hEvent : HANDLE ,) -> HRESULT , } }
};
}
