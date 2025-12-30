// Generated macro for macro_36818 (macro)
macro_rules! Depcrate_um_sapiddkmacro_36818 {
() => {
// Module: crate::um::sapiddk
// Provides: {"macro_36818"}
// Dependencies: {}
RIDL ! { # [uuid (0xfece8294 , 0x2be1 , 0x408f , 0x8e , 0x68 , 0x2d , 0xe3 , 0x77 , 0x09 , 0x2f , 0x0e)] interface ISpSRAlternates (ISpSRAlternatesVtbl) : IUnknown (IUnknownVtbl) { fn GetAlternates (pAltRequest : * mut SPPHRASEALTREQUEST , ppAlts : * mut * mut SPPHRASEALT , pcAlts : * mut ULONG ,) -> HRESULT , fn Commit (pAltRequest : * mut SPPHRASEALTREQUEST , pAlt : * mut SPPHRASEALT , ppvResultExtra : * mut c_void , pcbResultExtra : * mut ULONG ,) -> HRESULT , } }
};
}
