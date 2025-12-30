// Generated macro for macro_2049 (macro)
macro_rules! Depcrate_shared_d3d9macro_2049 {
() => {
// Module: crate::shared::d3d9
// Provides: {"macro_2049"}
// Dependencies: {}
RIDL ! { # [uuid (0x794950f2 , 0xadfc , 0x458a , 0x90 , 0x5e , 0x10 , 0xa1 , 0xb , 0xb , 0x50 , 0x3b)] interface IDirect3DSwapChain9 (IDirect3DSwapChain9Vtbl) : IUnknown (IUnknownVtbl) { fn Present (pSourceRect : * const RECT , pDestRect : * const RECT , hDestWindowOverride : HWND , pDirtyRegion : * const RGNDATA , dwFlags : DWORD ,) -> HRESULT , fn GetFrontBufferData (pDestSurface : * mut IDirect3DSurface9 ,) -> HRESULT , fn GetBackBuffer (iBackBuffer : UINT , Type : D3DBACKBUFFER_TYPE , ppBackBuffer : * mut * mut IDirect3DSurface9 ,) -> HRESULT , fn GetRasterStatus (pRasterStatus : * mut D3DRASTER_STATUS ,) -> HRESULT , fn GetDisplayMode (pMode : * mut D3DDISPLAYMODE ,) -> HRESULT , fn GetDevice (ppDevice : * mut * mut IDirect3DDevice9 ,) -> HRESULT , fn GetPresentParameters (pPresentationParameters : * mut D3DPRESENT_PARAMETERS ,) -> HRESULT , } }
};
}
