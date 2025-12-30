// Generated macro for macro_36631 (macro)
macro_rules! Depcrate_um_sapi51macro_36631 {
() => {
// Module: crate::um::sapi51
// Provides: {"macro_36631"}
// Dependencies: {}
RIDL ! { # [uuid (0xb238b6d5 , 0xf276 , 0x4c3d , 0xa6 , 0xc1 , 0x29 , 0x74 , 0x80 , 0x1c , 0x3c , 0xc2)] interface ISpeechPhraseAlternates (ISpeechPhraseAlternatesVtbl) : IDispatch (IDispatchVtbl) { fn get_Count (Count : * mut c_long ,) -> HRESULT , fn Item (Index : c_long , PhraseAlternate : * mut * mut ISpeechPhraseAlternate ,) -> HRESULT , fn get__NewEnum (EnumVARIANT : * mut * mut IUnknown ,) -> HRESULT , } }
};
}
