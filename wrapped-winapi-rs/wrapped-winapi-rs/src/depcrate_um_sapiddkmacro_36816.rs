// Generated macro for macro_36816 (macro)
macro_rules! Depcrate_um_sapiddkmacro_36816 {
() => {
// Module: crate::um::sapiddk
// Provides: {"macro_36816"}
// Dependencies: {}
RIDL ! { # [uuid (0x7ba627d8 , 0x33f9 , 0x4375 , 0x90 , 0xc5 , 0x99 , 0x85 , 0xae , 0xe5 , 0xed , 0xe5)] interface ISpSREngine2 (ISpSREngine2Vtbl) : ISpSREngine (ISpSREngineVtbl) { fn PrivateCallImmediate (pvEngineContext : * mut c_void , pInCallFrame : * const c_void , ulInCallFrameSize : ULONG , ppvCoMemResponse : * mut * mut c_void , pulResponseSize : * mut ULONG ,) -> HRESULT , fn SetAdaptationData2 (pvEngineContext : * mut c_void , pAdaptationData : * const WCHAR , cch : ULONG , pTopicName : LPCWSTR , eSettings : SPADAPTATIONSETTINGS , eRelevance : SPADAPTATIONRELEVANCE ,) -> HRESULT , fn SetGrammarPrefix (pvEngineGrammar : * mut c_void , pszPrefix : LPCWSTR , fIsPrefixRequired : BOOL ,) -> HRESULT , fn SetRulePriority (hRule : SPRULEHANDLE , pvClientRuleContext : * mut c_void , nRulePriority : c_int ,) -> HRESULT , fn EmulateRecognition (pPhrase : * mut ISpPhrase , dwCompareFlags : DWORD ,) -> HRESULT , fn SetSLMWeight (pvEngineGrammar : * mut c_void , flWeight : c_float ,) -> HRESULT , fn SetRuleWeight (hRule : SPRULEHANDLE , pvClientRuleContext : * mut c_void , flWeight : c_float ,) -> HRESULT , fn SetTrainingState (fDoingTraining : BOOL , fAdaptFromTrainingData : BOOL ,) -> HRESULT , fn ResetAcousticModelAdaptation () -> HRESULT , fn OnLoadCFG (pvEngineGrammar : * mut c_void , pvGrammarData : * const SPBINARYGRAMMAR , ulGrammarID : ULONG ,) -> HRESULT , fn OnUnloadCFG (pvEngineGrammar : * mut c_void , ulGrammarID : ULONG ,) -> HRESULT , } }
};
}
