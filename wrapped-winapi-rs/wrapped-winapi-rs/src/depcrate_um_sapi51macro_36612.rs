// Generated macro for macro_36612 (macro)
macro_rules! Depcrate_um_sapi51macro_36612 {
() => {
// Module: crate::um::sapi51
// Provides: {"macro_36612"}
// Dependencies: {}
RIDL ! { # [uuid (0xcff8e175 , 0x019e , 0x11d3 , 0xa0 , 0x8e , 0x00 , 0xc0 , 0x4f , 0x8e , 0xf9 , 0xb5)] interface ISpeechAudio (ISpeechAudioVtbl) : ISpeechBaseStream (ISpeechBaseStreamVtbl) { fn get_Status (Status : * mut * mut ISpeechAudioStatus ,) -> HRESULT , fn get_BufferInfo (BufferInfo : * mut * mut ISpeechAudioBufferInfo ,) -> HRESULT , fn get_DefaultFormat (StreamFormat : * mut * mut ISpeechAudioFormat ,) -> HRESULT , fn get_Volume (Volume : * mut c_long ,) -> HRESULT , fn put_Volume (Volume : c_long ,) -> HRESULT , fn get_BufferNotifySize (BufferNotifySize : * mut c_long ,) -> HRESULT , fn put_BufferNotifySize (BufferNotifySize : c_long ,) -> HRESULT , fn get_EventHandle (EventHandle : * mut c_long ,) -> HRESULT , fn SetState (State : SpeechAudioState ,) -> HRESULT , } }
};
}
