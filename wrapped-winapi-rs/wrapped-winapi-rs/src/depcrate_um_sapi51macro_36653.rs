// Generated macro for macro_36653 (macro)
macro_rules! Depcrate_um_sapi51macro_36653 {
() => {
// Module: crate::um::sapi51
// Provides: {"macro_36653"}
// Dependencies: {}
RIDL ! { # [uuid (0xc3e4f353 , 0x433f , 0x43d6 , 0x89 , 0xa1 , 0x6a , 0x62 , 0xa7 , 0x05 , 0x4c , 0x3d)] interface ISpeechPhoneConverter (ISpeechPhoneConverterVtbl) : IDispatch (IDispatchVtbl) { fn get_LanguageId (LanguageId : * mut SpeechLanguageId ,) -> HRESULT , fn put_LanguageId (LanguageId : SpeechLanguageId ,) -> HRESULT , fn PhoneToId (Phonemes : BSTR , IdArray : * mut VARIANT ,) -> HRESULT , fn IdToPhone (IdArray : VARIANT , Phonemes : * mut BSTR ,) -> HRESULT , } }
};
}
