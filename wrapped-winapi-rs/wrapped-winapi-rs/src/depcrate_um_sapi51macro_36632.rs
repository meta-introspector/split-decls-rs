// Generated macro for macro_36632 (macro)
macro_rules! Depcrate_um_sapi51macro_36632 {
() => {
// Module: crate::um::sapi51
// Provides: {"macro_36632"}
// Dependencies: {}
RIDL ! { # [uuid (0x961559cf , 0x4e67 , 0x4662 , 0x8b , 0xf0 , 0xd9 , 0x3f , 0x1f , 0xcd , 0x61 , 0xb3)] interface ISpeechPhraseInfo (ISpeechPhraseInfoVtbl) : IDispatch (IDispatchVtbl) { fn get_LanguageId (LanguageId : * mut c_long ,) -> HRESULT , fn get_GrammarId (GrammarId : * mut VARIANT ,) -> HRESULT , fn get_StartTime (StartTime : * mut VARIANT ,) -> HRESULT , fn get_AudioStreamPosition (AudioStreamPosition : * mut VARIANT ,) -> HRESULT , fn get_AudioSizeBytes (pAudioSizeBytes : * mut c_long ,) -> HRESULT , fn get_RetainedSizeBytes (RetainedSizeBytes : * mut c_long ,) -> HRESULT , fn get_AudioSizeTime (AudioSizeTime : * mut c_long ,) -> HRESULT , fn get_Rule (Rule : * mut * mut ISpeechPhraseRule ,) -> HRESULT , fn get_Properties (Properties : * mut * mut ISpeechPhraseProperties ,) -> HRESULT , fn get_Elements (Elements : * mut * mut ISpeechPhraseElements ,) -> HRESULT , fn get_Replacements (Replacements : * mut * mut ISpeechPhraseReplacements ,) -> HRESULT , fn get_EngineId (EngineIdGuid : * mut BSTR ,) -> HRESULT , fn get_EnginePrivateData (PrivateData : * mut VARIANT ,) -> HRESULT , fn SaveToMemory (PhraseBlock : * mut VARIANT ,) -> HRESULT , fn GetText (StartElement : c_long , Elements : c_long , UseReplacements : VARIANT_BOOL , Text : * mut BSTR ,) -> HRESULT , fn GetDisplayAttributes (StartElement : c_long , Elements : c_long , UseReplacements : VARIANT_BOOL , DisplayAttributes : * mut SpeechDisplayAttributes ,) -> HRESULT , } }
};
}
