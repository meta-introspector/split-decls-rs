// Generated macro for macro_36643 (macro)
macro_rules! Depcrate_um_sapi51macro_36643 {
() => {
// Module: crate::um::sapi51
// Provides: {"macro_36643"}
// Dependencies: {}
RIDL ! { # [uuid (0x4e5b933c , 0xc9be , 0x48ed , 0x88 , 0x42 , 0x1e , 0xe5 , 0x1b , 0xb1 , 0xd4 , 0xff)] interface ISpeechLexiconWord (ISpeechLexiconWordVtbl) : IDispatch (IDispatchVtbl) { fn get_LangId (LangId : * mut SpeechLanguageId ,) -> HRESULT , fn get_Type (WordType : * mut SpeechWordType ,) -> HRESULT , fn get_Word (Word : * mut BSTR ,) -> HRESULT , fn get_Pronunciations (Pronunciations : * mut * mut ISpeechLexiconPronunciations ,) -> HRESULT , } }
};
}
