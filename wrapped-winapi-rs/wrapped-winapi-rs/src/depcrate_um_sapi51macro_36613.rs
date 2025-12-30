// Generated macro for macro_36613 (macro)
macro_rules! Depcrate_um_sapi51macro_36613 {
() => {
// Module: crate::um::sapi51
// Provides: {"macro_36613"}
// Dependencies: {}
RIDL ! { # [uuid (0x3c76af6d , 0x1fd7 , 0x4831 , 0x81 , 0xd1 , 0x3b , 0x71 , 0xd5 , 0xa1 , 0x3c , 0x44)] interface ISpeechMMSysAudio (ISpeechMMSysAudioVtbl) : ISpeechAudio (ISpeechAudioVtbl) { fn get_DeviceId (DeviceId : * mut c_long ,) -> HRESULT , fn put_DeviceId (DeviceId : c_long ,) -> HRESULT , fn get_LineId (LineId : * mut c_long ,) -> HRESULT , fn put_LineId (LineId : c_long ,) -> HRESULT , fn get_MMHandle (Handle : * mut c_long ,) -> HRESULT , } }
};
}
