// Generated macro for macro_36623 (macro)
macro_rules! Depcrate_um_sapi51macro_36623 {
() => {
// Module: crate::um::sapi51
// Provides: {"macro_36623"}
// Dependencies: {}
RIDL ! { # [uuid (0x6ffa3b44 , 0xfc2d , 0x40d1 , 0x8a , 0xfc , 0x32 , 0x91 , 0x1c , 0x7f , 0x1a , 0xd1)] interface ISpeechGrammarRules (ISpeechGrammarRulesVtbl) : IDispatch (IDispatchVtbl) { fn get_Count (Count : * mut c_long ,) -> HRESULT , fn FindRule (RuleNameOrId : VARIANT , Rule : * mut * mut ISpeechGrammarRule ,) -> HRESULT , fn Item (Index : c_long , Rule : * mut * mut ISpeechGrammarRule ,) -> HRESULT , fn get__NewEnum (EnumVARIANT : * mut * mut IUnknown ,) -> HRESULT , fn get_Dynamic (Dynamic : * mut VARIANT_BOOL ,) -> HRESULT , fn Add (RuleName : BSTR , Attributes : SpeechRuleAttributes , RuleId : c_long , Rule : * mut * mut ISpeechGrammarRule ,) -> HRESULT , fn Commit () -> HRESULT , fn CommitAndSave (ErrorText : * mut BSTR , SaveStream : * mut VARIANT ,) -> HRESULT , } }
};
}
