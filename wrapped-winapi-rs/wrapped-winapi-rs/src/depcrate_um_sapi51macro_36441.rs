// Generated macro for macro_36441 (macro)
macro_rules! Depcrate_um_sapi51macro_36441 {
() => {
// Module: crate::um::sapi51
// Provides: {"macro_36441"}
// Dependencies: {}
RIDL ! { # [uuid (0x15806f6e , 0x1d70 , 0x4b48 , 0x98 , 0xe6 , 0x3b , 0x1a , 0x00 , 0x75 , 0x09 , 0xab)] interface ISpMMSysAudio (ISpMMSysAudioVtbl) : ISpAudio (ISpAudioVtbl) { fn GetDeviceId (puDeviceId : * mut UINT ,) -> HRESULT , fn SetDeviceId (uDeviceId : UINT ,) -> HRESULT , fn GetMMHandle (pHandle : * mut * mut c_void ,) -> HRESULT , fn GetLineId (puLineId : * mut UINT ,) -> HRESULT , fn SetLineId (uLineId : UINT ,) -> HRESULT , } }
};
}
