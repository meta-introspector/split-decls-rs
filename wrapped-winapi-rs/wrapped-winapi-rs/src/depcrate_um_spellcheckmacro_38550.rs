// Generated macro for macro_38550 (macro)
macro_rules! Depcrate_um_spellcheckmacro_38550 {
() => {
// Module: crate::um::spellcheck
// Provides: {"macro_38550"}
// Dependencies: {}
RIDL ! { # [uuid (0xb6fd0b71 , 0xe2bc , 0x4653 , 0x8d , 0x05 , 0xf1 , 0x97 , 0xe4 , 0x12 , 0x77 , 0x0b)] interface ISpellChecker (ISpellCheckerVtbl) : IUnknown (IUnknownVtbl) { fn get_LanguageTag (value : * mut LPWSTR ,) -> HRESULT , fn Check (text : LPCWSTR , value : * mut * mut IEnumSpellingError ,) -> HRESULT , fn Suggest (word : LPCWSTR , value : * mut * mut IEnumString ,) -> HRESULT , fn Add (word : LPCWSTR ,) -> HRESULT , fn Ignore (word : LPCWSTR ,) -> HRESULT , fn AutoCorrect (from : LPCWSTR , to : LPCWSTR ,) -> HRESULT , fn GetOptionValue (optionId : LPCWSTR , value : * mut BYTE ,) -> HRESULT , fn Get_OptionIds (value : * mut * mut IEnumString ,) -> HRESULT , fn Get_Id (value : * mut LPWSTR ,) -> HRESULT , fn Get_LocalizedName (value : * mut LPWSTR ,) -> HRESULT , fn add_SpellCheckerChanged (handler : * const ISpellCheckerChangedEventHandler , eventCookie : * mut DWORD ,) -> HRESULT , fn remove_SpellCheckerChanged (eventCookie : DWORD ,) -> HRESULT , fn GetOptionDescription (optionId : LPCWSTR , value : * mut * mut IOptionDescription ,) -> HRESULT , fn ComprehensiveCheck (text : LPCWSTR , value : * mut * mut IEnumSpellingError ,) -> HRESULT , } }
};
}
