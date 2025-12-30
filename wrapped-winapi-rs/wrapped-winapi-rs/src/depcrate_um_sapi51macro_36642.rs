// Generated macro for macro_36642 (macro)
macro_rules! Depcrate_um_sapi51macro_36642 {
() => {
// Module: crate::um::sapi51
// Provides: {"macro_36642"}
// Dependencies: {}
RIDL ! { # [uuid (0x8d199862 , 0x415e , 0x47d5 , 0xac , 0x4f , 0xfa , 0xa6 , 0x08 , 0xb4 , 0x24 , 0xe6)] interface ISpeechLexiconWords (ISpeechLexiconWordsVtbl) : IDispatch (IDispatchVtbl) { fn get_Count (Count : * mut c_long ,) -> HRESULT , fn Item (Index : c_long , Word : * mut * mut ISpeechLexiconWord ,) -> HRESULT , fn get__NewEnum (EnumVARIANT : * mut * mut IUnknown ,) -> HRESULT , } }
};
}
