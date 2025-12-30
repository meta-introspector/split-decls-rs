// Generated macro for macro_36639 (macro)
macro_rules! Depcrate_um_sapi51macro_36639 {
() => {
// Module: crate::um::sapi51
// Provides: {"macro_36639"}
// Dependencies: {}
RIDL ! { # [uuid (0xa7bfe112 , 0xa4a0 , 0x48d9 , 0xb6 , 0x02 , 0xc3 , 0x13 , 0x84 , 0x3f , 0x69 , 0x64)] interface ISpeechPhraseRule (ISpeechPhraseRuleVtbl) : IDispatch (IDispatchVtbl) { fn get_Name (Name : * mut BSTR ,) -> HRESULT , fn get_Id (Id : * mut c_long ,) -> HRESULT , fn get_FirstElement (FirstElement : * mut c_long ,) -> HRESULT , fn get_NumberOfElements (NumberOfElements : * mut c_long ,) -> HRESULT , fn get_Parent (Parent : * mut * mut ISpeechPhraseRule ,) -> HRESULT , fn get_Children (Children : * mut * mut ISpeechPhraseRules ,) -> HRESULT , fn get_Confidence (ActualConfidence : * mut SpeechEngineConfidence ,) -> HRESULT , fn get_EngineConfidence (Confidence : * mut c_float ,) -> HRESULT , } }
};
}
