// Generated macro for macro_3487 (macro)
macro_rules! Depcrate_shared_dxgi1_5macro_3487 {
() => {
// Module: crate::shared::dxgi1_5
// Provides: {"macro_3487"}
// Dependencies: {}
RIDL ! { # [uuid (0x95b4f95f , 0xd8da , 0x4ca4 , 0x9e , 0xe6 , 0x3b , 0x76 , 0xd5 , 0x96 , 0x8a , 0x10)] interface IDXGIDevice4 (IDXGIDevice4Vtbl) : IDXGIDevice3 (IDXGIDevice3Vtbl) { fn OfferResources1 (NumResources : UINT , ppResources : * mut * mut IDXGIResource , Priority : DXGI_OFFER_RESOURCE_PRIORITY , Flags : UINT ,) -> HRESULT , fn ReclaimResources1 (NumResources : UINT , ppResources : * mut * mut IDXGIResource , pResults : * mut DXGI_RECLAIM_RESOURCE_RESULTS ,) -> HRESULT , } }
};
}
