// Generated macro for macro_36605 (macro)
macro_rules! Depcrate_um_sapi51macro_36605 {
() => {
// Module: crate::um::sapi51
// Provides: {"macro_36605"}
// Dependencies: {}
RIDL ! { # [uuid (0xc62d9c91 , 0x7458 , 0x47f6 , 0x86 , 0x2d , 0x1e , 0xf8 , 0x6f , 0xb0 , 0xb2 , 0x78)] interface ISpeechAudioStatus (ISpeechAudioStatusVtbl) : IDispatch (IDispatchVtbl) { fn get_FreeBufferSpace (FreeBufferSpace : * mut c_long ,) -> HRESULT , fn get_NonBlockingIO (NonBlockingIO : * mut c_long ,) -> HRESULT , fn get_State (State : * mut SpeechAudioState ,) -> HRESULT , fn get_CurrentSeekPosition (CurrentSeekPosition : * mut VARIANT ,) -> HRESULT , fn get_CurrentDevicePosition (CurrentDevicePosition : * mut VARIANT ,) -> HRESULT , } }
};
}
