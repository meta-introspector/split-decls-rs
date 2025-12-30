// Generated macro for macro_3317 (macro)
macro_rules! Depcrate_shared_dxgimacro_3317 {
() => {
// Module: crate::shared::dxgi
// Provides: {"macro_3317"}
// Dependencies: {}
RIDL ! { # [uuid (0x035f3ab4 , 0x482e , 0x4e50 , 0xb4 , 0x1f , 0x8a , 0x7f , 0x8b , 0xd8 , 0x96 , 0x0b)] interface IDXGIResource (IDXGIResourceVtbl) : IDXGIDeviceSubObject (IDXGIDeviceSubObjectVtbl) { fn GetSharedHandle (pSharedHandle : * mut HANDLE ,) -> HRESULT , fn GetUsage (pUsage : * mut DXGI_USAGE ,) -> HRESULT , fn SetEvictionPriority (EvictionPriority : UINT ,) -> HRESULT , fn GetEvictionPriority (pEvictionPriority : * mut UINT ,) -> HRESULT , } }
};
}
