// Generated macro for macro_36814 (macro)
macro_rules! Depcrate_um_sapiddkmacro_36814 {
() => {
// Module: crate::um::sapiddk
// Provides: {"macro_36814"}
// Dependencies: {}
RIDL ! { # [uuid (0x7bc6e012 , 0x684a , 0x493e , 0xbd , 0xd4 , 0x2b , 0xf5 , 0xfb , 0xf4 , 0x8c , 0xfe)] interface ISpSREngineSite2 (ISpSREngineSite2Vtbl) : ISpSREngineSite (ISpSREngineSiteVtbl) { fn AddEventEx (pEvent : * const SPEVENTEX , hSAPIRecoContext : SPRECOCONTEXTHANDLE ,) -> HRESULT , fn UpdateRecoPosEx (ullCurrentRecoPos : ULONGLONG , ullCurrentRecoTime : ULONGLONG ,) -> HRESULT , fn GetRuleTransition (ulGrammarID : ULONG , RuleIndex : ULONG , pTrans : * mut SPTRANSITIONENTRY ,) -> HRESULT , fn RecognitionEx (pResultInfo : * const SPRECORESULTINFOEX ,) -> HRESULT , } }
};
}
