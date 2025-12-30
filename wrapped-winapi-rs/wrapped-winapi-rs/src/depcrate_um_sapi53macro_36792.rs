// Generated macro for macro_36792 (macro)
macro_rules! Depcrate_um_sapi53macro_36792 {
() => {
// Module: crate::um::sapi53
// Provides: {"macro_36792"}
// Dependencies: {}
RIDL ! { # [uuid (0x6d60eb64 , 0xaced , 0x40a6 , 0xbb , 0xf3 , 0x4e , 0x55 , 0x7f , 0x71 , 0xde , 0xe2)] interface ISpeechRecoResultDispatch (ISpeechRecoResultDispatchVtbl) : IDispatch (IDispatchVtbl) { fn get_RecoContext (RecoContext : * mut ISpeechRecoContext ,) -> HRESULT , fn get_Times (Times : * mut ISpeechRecoResultTimes ,) -> HRESULT , fn putref_AudioFormat (Format : * mut ISpeechAudioFormat ,) -> HRESULT , fn get_AudioFormat (Format : * mut * mut ISpeechAudioFormat ,) -> HRESULT , fn get_PhraseInfo (PhraseInfo : * mut * mut ISpeechPhraseInfo ,) -> HRESULT , fn Alternates (RequestCount : c_long , StartElement : c_long , Elements : c_long , Alternates : * mut * mut ISpeechPhraseAlternates ,) -> HRESULT , fn Audio (StartElement : c_long , Elements : c_long , Stream : * mut * mut ISpeechMemoryStream ,) -> HRESULT , fn SpeakAudio (StartElement : c_long , Elements : c_long , Flags : SpeechVoiceSpeakFlags , StreamNumber : * mut c_long ,) -> HRESULT , fn SaveToMemory (ResultBlock : * mut VARIANT ,) -> HRESULT , fn DiscardResultInfo (ValueTypes : SpeechDiscardType ,) -> HRESULT , fn GetXMLResult (Options : SPXMLRESULTOPTIONS , pResult : * mut BSTR ,) -> HRESULT , fn GetXMLErrorInfo (LineNumber : * mut c_long , ScriptLine : * mut BSTR , Source : * mut BSTR , Description : * mut BSTR , ResultCode : * mut HRESULT , IsError : * mut VARIANT_BOOL ,) -> HRESULT , fn SetTextFeedback (Feedback : BSTR , WasSuccessful : VARIANT_BOOL ,) -> HRESULT , } }
};
}
