// Generated macro for macro_36851 (macro)
macro_rules! Depcrate_um_sapiddk51macro_36851 {
() => {
// Module: crate::um::sapiddk51
// Provides: {"macro_36851"}
// Dependencies: {}
RIDL ! { # [uuid (0x88a3342a , 0x0bed , 0x4834 , 0x92 , 0x2b , 0x88 , 0xd4 , 0x31 , 0x73 , 0x16 , 0x2f)] interface ISpPhraseBuilder (ISpPhraseBuilderVtbl) : ISpPhrase (ISpPhraseVtbl) { fn InitFromPhrase (pPhrase : * const SPPHRASE ,) -> HRESULT , fn InitFromSerializedPhrase (pPhrase : * const SPSERIALIZEDPHRASE ,) -> HRESULT , fn AddElements (cElements : ULONG , pElement : * const SPPHRASEELEMENT ,) -> HRESULT , fn AddRules (hParent : SPPHRASERULEHANDLE , pRule : * const SPPHRASERULE , phNewRule : * mut SPPHRASERULEHANDLE ,) -> HRESULT , fn AddProperties (hParent : SPPHRASEPROPERTYHANDLE , pProperty : * const SPPHRASEPROPERTY , phNewProperty : * mut SPPHRASEPROPERTYHANDLE ,) -> HRESULT , fn AddReplacements (cReplacements : ULONG , pReplacements : * const SPPHRASEREPLACEMENT ,) -> HRESULT , } }
};
}
