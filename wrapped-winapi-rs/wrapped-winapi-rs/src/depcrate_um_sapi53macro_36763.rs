// Generated macro for macro_36763 (macro)
macro_rules! Depcrate_um_sapi53macro_36763 {
() => {
// Module: crate::um::sapi53
// Provides: {"macro_36763"}
// Dependencies: {}
RIDL ! { # [uuid (0xc8d7c7e2 , 0x0dde , 0x44b7 , 0xaf , 0xe3 , 0xb0 , 0xc9 , 0x91 , 0xfb , 0xeb , 0x5e)] interface ISpDisplayAlternates (ISpDisplayAlternatesVtbl) : IUnknown (IUnknownVtbl) { fn GetDisplayAlternates (pPhrase : * const SPDISPLAYPHRASE , cRequestCount : ULONG , ppCoMemPhrases : * mut * mut SPDISPLAYPHRASE , pcPhrasesReturned : * mut ULONG ,) -> HRESULT , fn SetFullStopTrailSpace (ulTrailSpace : ULONG ,) -> HRESULT , } }
};
}
