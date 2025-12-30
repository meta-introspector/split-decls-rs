// Generated macro for macro_36633 (macro)
macro_rules! Depcrate_um_sapi51macro_36633 {
() => {
// Module: crate::um::sapi51
// Provides: {"macro_36633"}
// Dependencies: {}
RIDL ! { # [uuid (0xe6176f96 , 0xe373 , 0x4801 , 0xb2 , 0x23 , 0x3b , 0x62 , 0xc0 , 0x68 , 0xc0 , 0xb4)] interface ISpeechPhraseElement (ISpeechPhraseElementVtbl) : IDispatch (IDispatchVtbl) { fn get_AudioTimeOffset (AudioTimeOffset : * mut c_long ,) -> HRESULT , fn get_AudioSizeTime (AudioSizeTime : * mut c_long ,) -> HRESULT , fn get_AudioStreamOffset (AudioStreamOffset : * mut c_long ,) -> HRESULT , fn get_AudioSizeBytes (AudioSizeBytes : * mut c_long ,) -> HRESULT , fn get_RetainedStreamOffset (RetainedStreamOffset : * mut c_long ,) -> HRESULT , fn get_RetainedSizeBytes (RetainedSizeBytes : * mut c_long ,) -> HRESULT , fn get_DisplayText (DisplayText : * mut BSTR ,) -> HRESULT , fn get_LexicalForm (LexicalForm : * mut BSTR ,) -> HRESULT , fn get_Pronunciation (Pronunciation : * mut VARIANT ,) -> HRESULT , fn get_DisplayAttributes (DisplayAttributes : * mut SpeechDisplayAttributes ,) -> HRESULT , fn get_RequiredConfidence (RequiredConfidence : * mut SpeechEngineConfidence ,) -> HRESULT , fn get_ActualConfidence (ActualConfidence : * mut SpeechEngineConfidence ,) -> HRESULT , fn get_EngineConfidence (EngineConfident : * mut c_float ,) -> HRESULT , } }
};
}
