// Generated macro for macro_28334 (macro)
macro_rules! Depcrate_um_dxvahdmacro_28334 {
() => {
// Module: crate::um::dxvahd
// Provides: {"macro_28334"}
// Dependencies: {}
RIDL ! { # [uuid (0x95f12dfd , 0xd77e , 0x49be , 0x81 , 0x5f , 0x57 , 0xd5 , 0x79 , 0x63 , 0x4d , 0x6d)] interface IDXVAHD_Device (IDXVAHD_DeviceVtbl) : IUnknown (IUnknownVtbl) { fn CreateVideoSurface (Width : UINT , Height : UINT , Format : D3DFORMAT , Pool : D3DPOOL , Usage : DWORD , Type : DXVAHD_SURFACE_TYPE , NumSurfaces : UINT , ppSurfaces : * mut * mut IDirect3DSurface9 , pSharedHandle : * mut HANDLE ,) -> HRESULT , fn GetVideoProcessorDeviceCaps (pCaps : * mut DXVAHD_VPDEVCAPS ,) -> HRESULT , fn GetVideoProcessorOutputFormats (Count : UINT , pFormats : * mut D3DFORMAT ,) -> HRESULT , fn GetVideoProcessorInputFormats (Count : UINT , pFormats : * mut D3DFORMAT ,) -> HRESULT , fn GetVideoProcessorCaps (Count : UINT , pCaps : * mut DXVAHD_VPCAPS ,) -> HRESULT , fn GetVideoProcessorCustomRates (pVPGuid : * const GUID , Count : UINT , pRates : * mut DXVAHD_CUSTOM_RATE_DATA ,) -> HRESULT , fn GetVideoProcessorFilterRange (Filter : DXVAHD_FILTER , pRange : * mut DXVAHD_FILTER_RANGE_DATA ,) -> HRESULT , fn CreateVideoProcessor (pVPGuid : * const GUID , ppVideoProcessor : * mut * mut IDXVAHD_VideoProcessor ,) -> HRESULT , } }
};
}
