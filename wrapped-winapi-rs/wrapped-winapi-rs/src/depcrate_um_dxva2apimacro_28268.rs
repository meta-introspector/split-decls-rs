// Generated macro for macro_28268 (macro)
macro_rules! Depcrate_um_dxva2apimacro_28268 {
() => {
// Module: crate::um::dxva2api
// Provides: {"macro_28268"}
// Dependencies: {}
RIDL ! { # [uuid (0xf2b0810a , 0xfd00 , 0x43c9 , 0x91 , 0x8c , 0xdf , 0x94 , 0xe2 , 0xd8 , 0xef , 0x7d)] interface IDirectXVideoDecoder (IDirectXVideoDecoderVtbl) : IUnknown (IUnknownVtbl) { fn GetVideoDecoderService (ppService : * mut * mut IDirectXVideoDecoderService ,) -> HRESULT , fn GetCreationParameters (pDeviceGuid : * mut GUID , pVideoDesc : * mut DXVA2_VideoDesc , pConfig : * mut DXVA2_ConfigPictureDecode , pDecoderRenderTargets : * mut * mut * mut IDirect3DSurface9 , pNumSurfaces : * mut UINT ,) -> HRESULT , fn GetBuffer (BufferType : UINT , ppBuffer : * mut * mut c_void , pBufferSize : * mut UINT ,) -> HRESULT , fn ReleaseBuffer (BufferType : UINT ,) -> HRESULT , fn BeginFrame (pRenderTarget : * mut IDirect3DSurface9 , pvPVPData : * mut c_void ,) -> HRESULT , fn EndFrame (pHandleComplete : * mut HANDLE ,) -> HRESULT , fn Execute (pExecuteParams : * const DXVA2_DecodeExecuteParams ,) -> HRESULT , } }
};
}
