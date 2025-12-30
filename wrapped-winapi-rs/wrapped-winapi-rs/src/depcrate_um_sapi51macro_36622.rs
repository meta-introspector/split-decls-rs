// Generated macro for macro_36622 (macro)
macro_rules! Depcrate_um_sapi51macro_36622 {
() => {
// Module: crate::um::sapi51
// Provides: {"macro_36622"}
// Dependencies: {}
RIDL ! { # [uuid (0xafe719cf , 0x5dd1 , 0x44f2 , 0x99 , 0x9c , 0x7a , 0x39 , 0x9f , 0x1c , 0xfc , 0xcc)] interface ISpeechGrammarRule (ISpeechGrammarRuleVtbl) : IDispatch (IDispatchVtbl) { fn get_Attributes (Attributes : * mut SpeechRuleAttributes ,) -> HRESULT , fn get_InitialState (State : * mut * mut ISpeechGrammarRuleState ,) -> HRESULT , fn get_Name (Name : * mut BSTR ,) -> HRESULT , fn get_Id (Id : * mut c_long ,) -> HRESULT , fn Clear () -> HRESULT , fn AddResource (ResourceName : BSTR , ResourceValue : BSTR ,) -> HRESULT , fn AddState (State : * mut * mut ISpeechGrammarRuleState ,) -> HRESULT , } }
};
}
