// Generated macro for macro_19840 (macro)
macro_rules! Depcrate_um_audioclientmacro_19840 {
() => {
// Module: crate::um::audioclient
// Provides: {"macro_19840"}
// Dependencies: {}
RIDL ! { # [uuid (0xc8adbd64 , 0xe71e , 0x48a0 , 0xa4 , 0xde , 0x18 , 0x5c , 0x39 , 0x5c , 0xd3 , 0x17)] interface IAudioCaptureClient (IAudioCaptureClientVtbl) : IUnknown (IUnknownVtbl) { fn GetBuffer (ppData : * mut * mut BYTE , pNumFramesToRead : * mut UINT32 , pdwFlags : * mut DWORD , pu64DevicePosition : * mut UINT64 , pu64QPCPosition : * mut UINT64 ,) -> HRESULT , fn ReleaseBuffer (NumFramesRead : UINT32 ,) -> HRESULT , fn GetNextPacketSize (pNumFramesInNextPacket : * mut UINT32 ,) -> HRESULT , } }
};
}
