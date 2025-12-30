// Generated macro for macro_36630 (macro)
macro_rules! Depcrate_um_sapi51macro_36630 {
() => {
// Module: crate::um::sapi51
// Provides: {"macro_36630"}
// Dependencies: {}
RIDL ! { # [uuid (0x27864a2a , 0x2b9f , 0x4cb8 , 0x92 , 0xd3 , 0x0d , 0x27 , 0x22 , 0xfd , 0x1e , 0x73)] interface ISpeechPhraseAlternate (ISpeechPhraseAlternateVtbl) : IDispatch (IDispatchVtbl) { fn get_RecoResult (RecoResult : * mut * mut ISpeechRecoResult ,) -> HRESULT , fn get_StartElementInResult (StartElement : * mut c_long ,) -> HRESULT , fn get_NumberOfElementsInResult (NumberOfElements : * mut c_long ,) -> HRESULT , fn get_PhraseInfo (PhraseInfo : * mut * mut ISpeechPhraseInfo ,) -> HRESULT , fn Commit () -> HRESULT , } }
};
}
