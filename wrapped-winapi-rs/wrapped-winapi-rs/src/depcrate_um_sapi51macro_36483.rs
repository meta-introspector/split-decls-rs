// Generated macro for macro_36483 (macro)
macro_rules! Depcrate_um_sapi51macro_36483 {
() => {
// Module: crate::um::sapi51
// Provides: {"macro_36483"}
// Dependencies: {}
RIDL ! { # [uuid (0x1a5c0354 , 0xb621 , 0x4b5a , 0x87 , 0x91 , 0xd3 , 0x06 , 0xed , 0x37 , 0x9e , 0x53)] interface ISpPhrase (ISpPhraseVtbl) : IUnknown (IUnknownVtbl) { fn GetPhrase (ppCoMemPhrase : * mut * mut SPPHRASE ,) -> HRESULT , fn GetSerializedPhrase (ppCoMemPhrase : * mut * mut SPSERIALIZEDPHRASE ,) -> HRESULT , fn GetText (ulStart : ULONG , ulCount : ULONG , fUseTextReplacements : BOOL , ppszCoMemText : * mut LPWSTR , pbDisplayAttributes : * mut BYTE ,) -> HRESULT , fn Discard (dwValueTypes : DWORD ,) -> HRESULT , } }
};
}
