// Generated macro for macro_36640 (macro)
macro_rules! Depcrate_um_sapi51macro_36640 {
() => {
// Module: crate::um::sapi51
// Provides: {"macro_36640"}
// Dependencies: {}
RIDL ! { # [uuid (0x9047d593 , 0x01dd , 0x4b72 , 0x81 , 0xa3 , 0xe4 , 0xa0 , 0xca , 0x69 , 0xf4 , 0x07)] interface ISpeechPhraseRules (ISpeechPhraseRulesVtbl) : IDispatch (IDispatchVtbl) { fn get_Count (Count : * mut c_long ,) -> HRESULT , fn Item (Index : c_long , Rule : * mut * mut ISpeechPhraseRule ,) -> HRESULT , fn get__NewEnum (EnumVARIANT : * mut * mut IUnknown ,) -> HRESULT , } }
};
}
