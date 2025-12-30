// Generated macro for macro_19839 (macro)
macro_rules! Depcrate_um_audioclientmacro_19839 {
() => {
// Module: crate::um::audioclient
// Provides: {"macro_19839"}
// Dependencies: {}
RIDL ! { # [uuid (0xf294acfc , 0x3146 , 0x4483 , 0xa7 , 0xbf , 0xad , 0xdc , 0xa7 , 0xc2 , 0x60 , 0xe2)] interface IAudioRenderClient (IAudioRenderClientVtbl) : IUnknown (IUnknownVtbl) { fn GetBuffer (NumFramesRequested : UINT32 , ppData : * mut * mut BYTE ,) -> HRESULT , fn ReleaseBuffer (NumFramesWritten : UINT32 , dwFlags : DWORD ,) -> HRESULT , } }
};
}
