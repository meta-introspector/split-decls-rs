// Generated macro for macro_27206 (macro)
macro_rules! Depcrate_um_dcompmacro_27206 {
() => {
// Module: crate::um::dcomp
// Provides: {"macro_27206"}
// Dependencies: {}
RIDL ! { # [uuid (0xe334bc12 , 0x3937 , 0x4e02 , 0x85 , 0xeb , 0xfc , 0xf4 , 0xeb , 0x30 , 0xd2 , 0xc8)] interface IDCompositionSurfaceFactory (IDCompositionSurfaceFactoryVtbl) : IUnknown (IUnknownVtbl) { fn CreateSurface (width : UINT , height : UINT , pixelFormat : DXGI_FORMAT , alphaMode : DXGI_ALPHA_MODE , surface : * mut * mut IDCompositionSurface ,) -> HRESULT , fn CreateVirtualSurface (initialWidth : UINT , initialHeight : UINT , pixelFormat : DXGI_FORMAT , alphaMode : DXGI_ALPHA_MODE , virtualSurface : * mut * mut IDCompositionVirtualSurface ,) -> HRESULT , } }
};
}
