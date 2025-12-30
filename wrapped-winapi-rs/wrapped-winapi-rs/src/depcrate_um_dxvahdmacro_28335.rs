// Generated macro for macro_28335 (macro)
macro_rules! Depcrate_um_dxvahdmacro_28335 {
() => {
// Module: crate::um::dxvahd
// Provides: {"macro_28335"}
// Dependencies: {}
RIDL ! { # [uuid (0x95f4edf4 , 0x6e03 , 0x4cd7 , 0xbe , 0x1b , 0x30 , 0x75 , 0xd6 , 0x65 , 0xaa , 0x52)] interface IDXVAHD_VideoProcessor (IDXVAHD_VideoProcessorVtbl) : IUnknown (IUnknownVtbl) { fn SetVideoProcessBltState (State : DXVAHD_BLT_STATE , DataSize : UINT , pData : * const c_void ,) -> HRESULT , fn GetVideoProcessBltState (State : DXVAHD_BLT_STATE , DataSize : UINT , pData : * mut c_void ,) -> HRESULT , fn SetVideoProcessStreamState (StreamNumber : UINT , State : DXVAHD_STREAM_STATE , DataSize : UINT , pData : * const c_void ,) -> HRESULT , fn GetVideoProcessStreamState (StreamNumber : UINT , State : DXVAHD_STREAM_STATE , DataSize : UINT , pData : * mut c_void ,) -> HRESULT , fn VideoProcessBltHD (pOutputSurface : * mut IDirect3DSurface9 , OutputFrame : UINT , StreamCount : UINT , pStreams : * const DXVAHD_STREAM_DATA ,) -> HRESULT , } }
};
}
