// Generated macro for macro_36628 (macro)
macro_rules! Depcrate_um_sapi51macro_36628 {
() => {
// Module: crate::um::sapi51
// Provides: {"macro_36628"}
// Dependencies: {}
RIDL ! { # [uuid (0xed2879cf , 0xced9 , 0x4ee6 , 0xa5 , 0x34 , 0xde , 0x01 , 0x91 , 0xd5 , 0x46 , 0x8d)] interface ISpeechRecoResult (ISpeechRecoResultVtbl) : IDispatch (IDispatchVtbl) { fn get_RecoContext (RecoContext : * mut * mut ISpeechRecoContext ,) -> HRESULT , fn get_Times (Times : * mut * mut ISpeechRecoResultTimes ,) -> HRESULT , fn putref_AudioFormat (Format : * mut ISpeechAudioFormat ,) -> HRESULT , fn get_AudioFormat (Format : * mut * mut ISpeechAudioFormat ,) -> HRESULT , fn get_PhraseInfo (PhraseInfo : * mut * mut ISpeechPhraseInfo ,) -> HRESULT , fn Alternates (RequestCount : c_long , StartElement : c_long , Elements : c_long , Alternates : * mut * mut ISpeechPhraseAlternates ,) -> HRESULT , fn Audio (StartElement : c_long , Elements : c_long , Stream : * mut * mut ISpeechMemoryStream ,) -> HRESULT , fn SpeakAudio (StartElement : c_long , Elements : c_long , Flags : SpeechVoiceSpeakFlags , StreamNumber : * mut c_long ,) -> HRESULT , fn SaveToMemory (ResultBlock : * mut VARIANT ,) -> HRESULT , fn DiscardResultInfo (ValueTypes : SpeechDiscardType ,) -> HRESULT , } }
};
}
