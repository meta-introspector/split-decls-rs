// Generated macro for macro_36606 (macro)
macro_rules! Depcrate_um_sapi51macro_36606 {
() => {
// Module: crate::um::sapi51
// Provides: {"macro_36606"}
// Dependencies: {}
RIDL ! { # [uuid (0xe6e9c590 , 0x3e18 , 0x40e3 , 0x82 , 0x99 , 0x06 , 0x1f , 0x98 , 0xbd , 0xe7 , 0xc7)] interface ISpeechAudioFormat (ISpeechAudioFormatVtbl) : IDispatch (IDispatchVtbl) { fn get_Type (AudioFormat : * mut SpeechAudioFormatType ,) -> HRESULT , fn put_Type (AudioFormat : SpeechAudioFormatType ,) -> HRESULT , fn get_Guid (Guid : * mut BSTR ,) -> HRESULT , fn put_Guid (Guid : BSTR ,) -> HRESULT , fn GetWaveFormatEx (SpeechWaveFormatEx : * mut * mut ISpeechWaveFormatEx ,) -> HRESULT , fn SetWaveFormatEx (SpeechWaveFormatEx : * mut ISpeechWaveFormatEx ,) -> HRESULT , } }
};
}
