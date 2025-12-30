// Generated macro for macro_36637 (macro)
macro_rules! Depcrate_um_sapi51macro_36637 {
() => {
// Module: crate::um::sapi51
// Provides: {"macro_36637"}
// Dependencies: {}
RIDL ! { # [uuid (0xce563d48 , 0x961e , 0x4732 , 0xa2 , 0xe1 , 0x37 , 0x8a , 0x42 , 0xb4 , 0x30 , 0xbe)] interface ISpeechPhraseProperty (ISpeechPhrasePropertyVtbl) : IDispatch (IDispatchVtbl) { fn get_Name (Name : * mut BSTR ,) -> HRESULT , fn get_Id (Id : * mut c_long ,) -> HRESULT , fn get_Value (Value : * mut VARIANT ,) -> HRESULT , fn get_FirstElement (FirstElement : * mut c_long ,) -> HRESULT , fn get_NumberOfElements (NumberOfElements : * mut c_long ,) -> HRESULT , fn get_EngineConfidence (Confidence : * mut c_float ,) -> HRESULT , fn get_Confidence (Confidence : * mut SpeechEngineConfidence ,) -> HRESULT , fn get_Parent (ParentProperty : * mut * mut ISpeechPhraseProperty ,) -> HRESULT , fn get_Children (Children : * mut * mut ISpeechPhraseProperties ,) -> HRESULT , } }
};
}
