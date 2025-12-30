// Generated macro for macro_36618 (macro)
macro_rules! Depcrate_um_sapi51macro_36618 {
() => {
// Module: crate::um::sapi51
// Provides: {"macro_36618"}
// Dependencies: {}
RIDL ! { # [uuid (0xbff9e781 , 0x53ec , 0x484e , 0xbb , 0x8a , 0x0e , 0x1b , 0x55 , 0x51 , 0xe3 , 0x5c)] interface ISpeechRecognizerStatus (ISpeechRecognizerStatusVtbl) : IDispatch (IDispatchVtbl) { fn get_AudioStatus (AudioStatus : * mut * mut ISpeechAudioStatus ,) -> HRESULT , fn get_CurrentStreamPosition (pCurrentStreamPos : * mut VARIANT ,) -> HRESULT , fn get_CurrentStreamNumber (StreamNumber : * mut c_long ,) -> HRESULT , fn get_NumberOfActiveRules (NumberOfActiveRules : * mut c_long ,) -> HRESULT , fn get_ClsidEngine (ClsidEngine : * mut BSTR ,) -> HRESULT , fn get_SupportedLanguages (SupportedLanguages : * mut VARIANT ,) -> HRESULT , } }
};
}
