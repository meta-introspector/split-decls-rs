// Generated macro for macro_36746 (macro)
macro_rules! Depcrate_um_sapi53macro_36746 {
() => {
// Module: crate::um::sapi53
// Provides: {"macro_36746"}
// Dependencies: {}
RIDL ! { # [uuid (0x4b37bc9e , 0x9ed6 , 0x44a3 , 0x93 , 0xd3 , 0x18 , 0xf0 , 0x22 , 0xb7 , 0x9e , 0xc3)] interface ISpRecoGrammar2 (ISpRecoGrammar2Vtbl) : IUnknown (IUnknownVtbl) { fn GetRules (ppCoMemRules : * mut * mut SPRULE , puNumRules : * mut UINT ,) -> HRESULT , fn LoadCmdFromFile2 (pszFileName : LPCWSTR , Options : SPLOADOPTIONS , pszSharingUri : LPCWSTR , pszBaseUri : LPCWSTR ,) -> HRESULT , fn LoadCmdFromMemory2 (pGrammar : * const SPBINARYGRAMMAR , Options : SPLOADOPTIONS , pszSharingUri : LPCWSTR , pszBaseUri : LPCWSTR ,) -> HRESULT , fn SetRulePriority (pszRuleName : LPCWSTR , ulRuleId : ULONG , nRulePriority : c_int ,) -> HRESULT , fn SetRuleWeight (pszRuleName : LPCWSTR , ulRuleId : ULONG , flWeight : c_float ,) -> HRESULT , fn SetDictationWeight (flWeight : c_float ,) -> HRESULT , fn SetGrammarLoader (pLoader : * mut ISpeechResourceLoader ,) -> HRESULT , fn SetSMLSecurityManager (pSMLSecurityManager : * mut IInternetSecurityManager ,) -> HRESULT , } }
};
}
