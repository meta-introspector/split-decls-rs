// Generated macro for macro_36625 (macro)
macro_rules! Depcrate_um_sapi51macro_36625 {
() => {
// Module: crate::um::sapi51
// Provides: {"macro_36625"}
// Dependencies: {}
RIDL ! { # [uuid (0xcafd1db1 , 0x41d1 , 0x4a06 , 0x98 , 0x63 , 0xe2 , 0xe8 , 0x1d , 0xa1 , 0x7a , 0x9a)] interface ISpeechGrammarRuleStateTransition (ISpeechGrammarRuleStateTransitionVtbl) : IDispatch (IDispatchVtbl) { fn get_Type (Type : * mut SpeechGrammarRuleStateTransitionType ,) -> HRESULT , fn get_Text (Text : * mut BSTR ,) -> HRESULT , fn get_Rule (Rule : * mut * mut ISpeechGrammarRule ,) -> HRESULT , fn get_Weight (Weight : * mut VARIANT ,) -> HRESULT , fn get_PropertyName (PropertyName : * mut BSTR ,) -> HRESULT , fn get_PropertyId (PropertyId : * mut c_long ,) -> HRESULT , fn get_PropertyValue (PropertyValue : * mut VARIANT ,) -> HRESULT , fn get_NextState (NextState : * mut * mut ISpeechGrammarRuleState ,) -> HRESULT , } }
};
}
