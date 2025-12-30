// Generated macro for macro_36608 (macro)
macro_rules! Depcrate_um_sapi51macro_36608 {
() => {
// Module: crate::um::sapi51
// Provides: {"macro_36608"}
// Dependencies: {}
RIDL ! { # [uuid (0x6450336f , 0x7d49 , 0x4ced , 0x80 , 0x97 , 0x49 , 0xd6 , 0xde , 0xe3 , 0x72 , 0x94)] interface ISpeechBaseStream (ISpeechBaseStreamVtbl) : IDispatch (IDispatchVtbl) { fn get_Format (AudioFormat : * mut * mut ISpeechAudioFormat ,) -> HRESULT , fn putref_Format (AudioFormat : * mut ISpeechAudioFormat ,) -> HRESULT , fn Read (Buffer : * mut VARIANT , NumberOfBytes : c_long , BytesRead : * mut c_long ,) -> HRESULT , fn Write (Buffer : VARIANT , BytesWritten : * mut c_long ,) -> HRESULT , fn Seek (Position : VARIANT , Origin : SpeechStreamSeekPositionType , NewPosition : * mut VARIANT ,) -> HRESULT , } }
};
}
