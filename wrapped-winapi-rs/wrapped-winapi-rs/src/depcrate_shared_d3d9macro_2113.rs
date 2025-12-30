// Generated macro for macro_2113 (macro)
macro_rules! Depcrate_shared_d3d9macro_2113 {
() => {
// Module: crate::shared::d3d9
// Provides: {"macro_2113"}
// Dependencies: {}
RIDL ! { # [uuid (0x91886caf , 0x1c3d , 0x4d2e , 0xa0 , 0xab , 0x3e , 0x4c , 0x7d , 0x8d , 0x33 , 0x3)] interface IDirect3DSwapChain9Ex (IDirect3DSwapChain9ExVtbl) : IDirect3DSwapChain9 (IDirect3DSwapChain9Vtbl) { fn GetLastPresentCount (pLastPresentCount : * mut UINT ,) -> HRESULT , fn GetPresentStats (pPresentationStatistics : * mut D3DPRESENTSTATS ,) -> HRESULT , fn GetDisplayModeEx (pMode : * mut D3DDISPLAYMODEEX , pRotation : * mut D3DDISPLAYROTATION ,) -> HRESULT , } }
};
}
