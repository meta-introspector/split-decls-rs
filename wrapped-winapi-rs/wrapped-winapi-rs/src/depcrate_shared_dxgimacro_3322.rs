// Generated macro for macro_3322 (macro)
macro_rules! Depcrate_shared_dxgimacro_3322 {
() => {
// Module: crate::shared::dxgi
// Provides: {"macro_3322"}
// Dependencies: {}
RIDL ! { # [uuid (0xcafcb56c , 0x6ac3 , 0x4889 , 0xbf , 0x47 , 0x9e , 0x23 , 0xbb , 0xd2 , 0x60 , 0xec)] interface IDXGISurface (IDXGISurfaceVtbl) : IDXGIDeviceSubObject (IDXGIDeviceSubObjectVtbl) { fn GetDesc (pDesc : * mut DXGI_SURFACE_DESC ,) -> HRESULT , fn Map (pLockedRect : * mut DXGI_MAPPED_RECT , MapFlags : UINT ,) -> HRESULT , fn Unmap () -> HRESULT , } }
};
}
