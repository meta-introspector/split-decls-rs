// Generated macro for macro_36873 (macro)
macro_rules! Depcrate_um_sapiddk51macro_36873 {
() => {
// Module: crate::um::sapiddk51
// Provides: {"macro_36873"}
// Dependencies: {}
RIDL ! { # [uuid (0xf3d3f926 , 0x11fc , 0x11d3 , 0xbb , 0x97 , 0x00 , 0xc0 , 0x4f , 0x8e , 0xe6 , 0xc0)] interface ISpCFGInterpreter (ISpCFGInterpreterVtbl) : IUnknown (IUnknownVtbl) { fn InitGrammar (pszGrammarName : LPCWSTR , pvGrammarData : * mut * const c_void ,) -> HRESULT , fn Interpret (pPhrase : * mut ISpPhraseBuilder , ulFirstElement : * const ULONG , ulCountOfElements : * const ULONG , pSite : * mut ISpCFGInterpreterSite ,) -> HRESULT , } }
};
}
