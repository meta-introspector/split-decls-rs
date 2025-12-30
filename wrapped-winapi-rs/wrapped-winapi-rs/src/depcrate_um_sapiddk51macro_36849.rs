// Generated macro for macro_36849 (macro)
macro_rules! Depcrate_um_sapiddk51macro_36849 {
() => {
// Module: crate::um::sapiddk51
// Provides: {"macro_36849"}
// Dependencies: {}
RIDL ! { # [uuid (0x3ddca27c , 0x665c , 0x4786 , 0x9f , 0x97 , 0x8c , 0x90 , 0xc3 , 0x48 , 0x8b , 0x61)] interface ISpGramCompBackend (ISpGramCompBackendVtbl) : ISpGrammarBuilder (ISpGrammarBuilderVtbl) { fn SetSaveObjects (pStream : * mut IStream , pErrorLog : * mut ISpErrorLog ,) -> HRESULT , fn InitFromBinaryGrammar (pBinaryData : * const SPBINARYGRAMMAR ,) -> HRESULT , } }
};
}
