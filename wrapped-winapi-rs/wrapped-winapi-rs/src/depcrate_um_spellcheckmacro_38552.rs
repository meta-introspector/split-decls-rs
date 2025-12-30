// Generated macro for macro_38552 (macro)
macro_rules! Depcrate_um_spellcheckmacro_38552 {
() => {
// Module: crate::um::spellcheck
// Provides: {"macro_38552"}
// Dependencies: {}
RIDL ! { # [uuid (0x8e018a9d , 0x2415 , 0x4677 , 0xbf , 0x08 , 0x79 , 0x4e , 0xa6 , 0x1f , 0x94 , 0xbb)] interface ISpellCheckerFactory (ISpellCheckerFactoryVtbl) : IUnknown (IUnknownVtbl) { fn SupportedLanguages (value : * mut * mut IEnumString ,) -> HRESULT , fn IsSupported (languageTag : LPCWSTR , value : * mut BOOL ,) -> HRESULT , fn CreateSpellChecker (languageTag : LPCWSTR , value : * mut * mut ISpellChecker ,) -> HRESULT , } }
};
}
