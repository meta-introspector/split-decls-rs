// Generated macro for macro_36624 (macro)
macro_rules! Depcrate_um_sapi51macro_36624 {
() => {
// Module: crate::um::sapi51
// Provides: {"macro_36624"}
// Dependencies: {}
RIDL ! { # [uuid (0xd4286f2c , 0xee67 , 0x45ae , 0xb9 , 0x28 , 0x28 , 0xd6 , 0x95 , 0x36 , 0x2e , 0xda)] interface ISpeechGrammarRuleState (ISpeechGrammarRuleStateVtbl) : IDispatch (IDispatchVtbl) { fn get_Rule (Rule : * mut * mut ISpeechGrammarRule ,) -> HRESULT , fn get_Transitions (Transitions : * mut * mut ISpeechGrammarRuleStateTransitions ,) -> HRESULT , fn AddWordTransition (DestState : * mut ISpeechGrammarRuleState , Words : BSTR , Separators : BSTR , Type : SpeechGrammarWordType , PropertyName : BSTR , PropertyId : c_long , PropertyValue : * mut VARIANT , Weight : c_float ,) -> HRESULT , fn AddRuleTransition (DestinationState : * mut ISpeechGrammarRuleState , Rule : * mut ISpeechGrammarRule , PropertyName : BSTR , PropertyId : c_long , PropertyValue : * mut VARIANT , Weight : c_float ,) -> HRESULT , fn AddSpecialTransition (DestinationState : * mut ISpeechGrammarRuleState , Type : SpeechSpecialTransitionType , PropertyName : BSTR , PropertyId : c_long , PropertyValue : * mut VARIANT , Weight : c_float ,) -> HRESULT , } }
};
}
