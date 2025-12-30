// Generated macro for macro_36755 (macro)
macro_rules! Depcrate_um_sapi53macro_36755 {
() => {
// Module: crate::um::sapi53
// Provides: {"macro_36755"}
// Dependencies: {}
RIDL ! { # [uuid (0xbead311c , 0x52ff , 0x437f , 0x94 , 0x64 , 0x6b , 0x21 , 0x05 , 0x4c , 0xa7 , 0x3d)] interface ISpRecoContext2 (ISpRecoContext2Vtbl) : IUnknown (IUnknownVtbl) { fn SetGrammarOptions (eGrammarOptions : DWORD ,) -> HRESULT , fn GetGrammarOptions (peGrammarOptions : * mut DWORD ,) -> HRESULT , fn SetAdaptationData2 (pAdaptationData : LPCWSTR , cch : ULONG , pTopicName : LPCWSTR , eAdaptationSettings : DWORD , eRelevance : SPADAPTATIONRELEVANCE ,) -> HRESULT , } }
};
}
