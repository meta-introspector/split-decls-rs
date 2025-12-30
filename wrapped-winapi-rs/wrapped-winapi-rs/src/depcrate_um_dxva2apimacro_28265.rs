// Generated macro for macro_28265 (macro)
macro_rules! Depcrate_um_dxva2apimacro_28265 {
() => {
// Module: crate::um::dxva2api
// Provides: {"macro_28265"}
// Dependencies: {}
RIDL ! { # [uuid (0xfc51a550 , 0xd5e7 , 0x11d9 , 0xaf , 0x55 , 0x00 , 0x05 , 0x4e , 0x43 , 0xff , 0x02)] interface IDirectXVideoAccelerationService (IDirectXVideoAccelerationServiceVtbl) : IUnknown (IUnknownVtbl) { fn CreateSurface (Width : UINT , Height : UINT , BackBuffers : UINT , Format : D3DFORMAT , Pool : D3DPOOL , Usage : DWORD , DxvaType : DWORD , ppSurface : * mut * mut IDirect3DSurface9 , pSharedHandle : * mut HANDLE ,) -> HRESULT , } }
};
}
