// Generated macro for macro_28269 (macro)
macro_rules! Depcrate_um_dxva2apimacro_28269 {
() => {
// Module: crate::um::dxva2api
// Provides: {"macro_28269"}
// Dependencies: {}
RIDL ! { # [uuid (0x8c3a39f0 , 0x916e , 0x4690 , 0x80 , 0x4f , 0x4c , 0x80 , 0x01 , 0x35 , 0x5d , 0x25)] interface IDirectXVideoProcessor (IDirectXVideoProcessorVtbl) : IUnknown (IUnknownVtbl) { fn GetVideoProcessorService (ppService : * mut * mut IDirectXVideoProcessorService ,) -> HRESULT , fn GetCreationParameters (pDeviceGuid : * mut GUID , pVideoDesc : * mut DXVA2_VideoDesc , pRenderTargetFormat : * mut D3DFORMAT , pMaxNumSubStreams : * mut UINT ,) -> HRESULT , fn GetVideoProcessorCaps (pCaps : * mut DXVA2_VideoProcessorCaps ,) -> HRESULT , fn GetProcAmpRange (ProcAmpCap : UINT , pRange : * mut DXVA2_ValueRange ,) -> HRESULT , fn GetFilterPropertyRange (FilterSetting : UINT , pRange : * mut DXVA2_ValueRange ,) -> HRESULT , fn VideoProcessBlt (pRenderTarget : * mut IDirect3DSurface9 , pBltParams : * const DXVA2_VideoProcessBltParams , pSamples : * const DXVA2_VideoSample , NumSamples : UINT , pHandleComplete : * mut HANDLE ,) -> HRESULT , } }
};
}
