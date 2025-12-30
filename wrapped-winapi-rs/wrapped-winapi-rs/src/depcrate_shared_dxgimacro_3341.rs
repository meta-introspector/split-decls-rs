// Generated macro for macro_3341 (macro)
macro_rules! Depcrate_shared_dxgimacro_3341 {
() => {
// Module: crate::shared::dxgi
// Provides: {"macro_3341"}
// Dependencies: {}
RIDL ! { # [uuid (0x54ec77fa , 0x1377 , 0x44e6 , 0x8c , 0x32 , 0x88 , 0xfd , 0x5f , 0x44 , 0xc8 , 0x4c)] interface IDXGIDevice (IDXGIDeviceVtbl) : IDXGIObject (IDXGIObjectVtbl) { fn GetAdapter (pAdapter : * mut * mut IDXGIAdapter ,) -> HRESULT , fn CreateSurface (pDesc : * const DXGI_SURFACE_DESC , NumSurfaces : UINT , Usage : DXGI_USAGE , pSharedResource : * const DXGI_SHARED_RESOURCE , ppSurface : * mut * mut IDXGISurface ,) -> HRESULT , fn QueryResourceResidency (ppResources : * const * mut IUnknown , pResidencyStatus : * mut DXGI_RESIDENCY , NumResources : UINT ,) -> HRESULT , fn SetGPUThreadPriority (Priority : INT ,) -> HRESULT , fn GetGPUThreadPriority (pPriority : * mut INT ,) -> HRESULT , } }
};
}
