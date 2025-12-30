// Generated macro for macro_36760 (macro)
macro_rules! Depcrate_um_sapi53macro_36760 {
() => {
// Module: crate::um::sapi53
// Provides: {"macro_36760"}
// Dependencies: {}
RIDL ! { # [uuid (0xc360ce4b , 0x76d1 , 0x4214 , 0xad , 0x68 , 0x52 , 0x65 , 0x7d , 0x50 , 0x83 , 0xda)] interface ISpEnginePronunciation (ISpEnginePronunciationVtbl) : IUnknown (IUnknownVtbl) { fn Normalize (pszWord : LPCWSTR , pszLeftContext : LPCWSTR , pszRightContext : LPCWSTR , LangID : WORD , pNormalizationList : * mut SPNORMALIZATIONLIST ,) -> HRESULT , fn GetPronunciations (pszWord : LPCWSTR , pszLeftContext : LPCWSTR , pszRightContext : LPCWSTR , LangID : WORD , pEnginePronunciationList : * mut SPWORDPRONUNCIATIONLIST ,) -> HRESULT , } }
};
}
