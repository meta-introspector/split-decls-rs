// Generated macro for macro_36501 (macro)
macro_rules! Depcrate_um_sapi51macro_36501 {
() => {
// Module: crate::um::sapi51
// Provides: {"macro_36501"}
// Dependencies: {}
RIDL ! { # [uuid (0x8137828f , 0x591a , 0x4a42 , 0xbe , 0x58 , 0x49 , 0xea , 0x7e , 0xba , 0xac , 0x68)] interface ISpGrammarBuilder (ISpGrammarBuilderVtbl) : IUnknown (IUnknownVtbl) { fn ResetGrammar (NewLanguage : WORD ,) -> HRESULT , fn GetRule (pszRuleName : LPCWSTR , dwRuleId : DWORD , dwAttributes : DWORD , fCreateIfNotExist : BOOL , phInitialState : * mut SPSTATEHANDLE ,) -> HRESULT , fn ClearRule (hState : SPSTATEHANDLE ,) -> HRESULT , fn CreateNewState (hState : SPSTATEHANDLE , phState : * mut SPSTATEHANDLE ,) -> HRESULT , fn AddWordTransition (hFromState : SPSTATEHANDLE , hToState : SPSTATEHANDLE , psz : LPCWSTR , pszSeparators : LPCWSTR , eWordType : SPGRAMMARWORDTYPE , Weight : c_float , pPropInfo : * const SPPROPERTYINFO ,) -> HRESULT , fn AddRuleTransition (hFromState : SPSTATEHANDLE , hToState : SPSTATEHANDLE , hRule : SPSTATEHANDLE , Weight : c_float , pPropInfo : * const SPPROPERTYINFO ,) -> HRESULT , fn AddResource (hRuleState : SPSTATEHANDLE , pszResourceName : LPCWSTR , pszResourceValue : LPCWSTR ,) -> HRESULT , fn Commit (dwReserved : DWORD ,) -> HRESULT , } }
};
}
