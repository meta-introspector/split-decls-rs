// Generated macro for macro_36641 (macro)
macro_rules! Depcrate_um_sapi51macro_36641 {
() => {
// Module: crate::um::sapi51
// Provides: {"macro_36641"}
// Dependencies: {}
RIDL ! { # [uuid (0x3da7627a , 0xc7ae , 0x4b23 , 0x87 , 0x08 , 0x63 , 0x8c , 0x50 , 0x36 , 0x2c , 0x25)] interface ISpeechLexicon (ISpeechLexiconVtbl) : IDispatch (IDispatchVtbl) { fn get_GenerationId (GenerationId : * mut c_long ,) -> HRESULT , fn GetWords (Flags : SpeechLexiconType , GenerationID : * mut c_long , Words : * mut * mut ISpeechLexiconWords ,) -> HRESULT , fn AddPronunciation (bstrWord : BSTR , LangId : SpeechLanguageId , PartOfSpeech : SpeechPartOfSpeech , bstrPronunciation : BSTR ,) -> HRESULT , fn AddPronunciationByPhoneIds (bstrWord : BSTR , LangId : SpeechLanguageId , PartOfSpeech : SpeechPartOfSpeech , PhoneIds : * mut VARIANT ,) -> HRESULT , fn RemovePronunciation (bstrWord : BSTR , LangId : SpeechLanguageId , PartOfSpeech : SpeechPartOfSpeech , bstrPronunciation : BSTR ,) -> HRESULT , fn RemovePronunciationByPhoneIds (bstrWord : BSTR , LangId : SpeechLanguageId , PartOfSpeech : SpeechPartOfSpeech , PhoneIds : * mut VARIANT ,) -> HRESULT , fn GetPronunciations (bstrWord : BSTR , LangId : SpeechLanguageId , TypeFlags : SpeechLexiconType , ppPronunciations : * mut * mut ISpeechLexiconPronunciations ,) -> HRESULT , fn GetGenerationChange (GenerationID : * mut c_long , ppWords : * mut * mut ISpeechLexiconWords ,) -> HRESULT , } }
};
}
