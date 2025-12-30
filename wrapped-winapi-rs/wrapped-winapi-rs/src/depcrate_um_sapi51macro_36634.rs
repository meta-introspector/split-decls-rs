// Generated macro for macro_36634 (macro)
macro_rules! Depcrate_um_sapi51macro_36634 {
() => {
// Module: crate::um::sapi51
// Provides: {"macro_36634"}
// Dependencies: {}
RIDL ! { # [uuid (0x0626b328 , 0x3478 , 0x467d , 0xa0 , 0xb3 , 0xd0 , 0x85 , 0x3b , 0x93 , 0xdd , 0xa3)] interface ISpeechPhraseElements (ISpeechPhraseElementsVtbl) : IDispatch (IDispatchVtbl) { fn get_Count (Count : * mut c_long ,) -> HRESULT , fn Item (Index : c_long , Element : * mut * mut ISpeechPhraseElement ,) -> HRESULT , fn get__NewEnum (EnumVARIANT : * mut * mut IUnknown ,) -> HRESULT , } }
};
}
