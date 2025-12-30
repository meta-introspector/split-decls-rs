// Generated macro for macro_28266 (macro)
macro_rules! Depcrate_um_dxva2apimacro_28266 {
() => {
// Module: crate::um::dxva2api
// Provides: {"macro_28266"}
// Dependencies: {}
RIDL ! { # [uuid (0xfc51a551 , 0xd5e7 , 0x11d9 , 0xaf , 0x55 , 0x00 , 0x05 , 0x4e , 0x43 , 0xff , 0x02)] interface IDirectXVideoDecoderService (IDirectXVideoDecoderServiceVtbl) : IDirectXVideoAccelerationService (IDirectXVideoAccelerationServiceVtbl) { fn GetDecoderDeviceGuids (pCount : * mut UINT , pGuids : * mut * mut GUID ,) -> HRESULT , fn GetDecoderRenderTargets (Guid : REFGUID , pCount : * mut UINT , pFormats : * mut * mut D3DFORMAT ,) -> HRESULT , fn GetDecoderConfigurations (Guid : REFGUID , pVideoDesc : * const DXVA2_VideoDesc , pReserved : * mut c_void , pCount : * mut UINT , ppConfigs : * mut * mut DXVA2_ConfigPictureDecode ,) -> HRESULT , fn CreateVideoDecoder (Guid : REFGUID , pVideoDesc : * const DXVA2_VideoDesc , pConfig : * const DXVA2_ConfigPictureDecode , ppDecoderRenderTargets : * mut * mut IDirect3DSurface9 , NumRenderTargets : UINT , ppDecode : * mut * mut IDirectXVideoDecoder ,) -> HRESULT , } }
};
}
