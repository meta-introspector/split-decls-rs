// Generated macro for macro_36728 (macro)
macro_rules! Depcrate_um_sapi53macro_36728 {
() => {
// Module: crate::um::sapi53
// Provides: {"macro_36728"}
// Dependencies: {}
RIDL ! { # [uuid (0xf264da52 , 0xe457 , 0x4696 , 0xb8 , 0x56 , 0xa7 , 0x37 , 0xb7 , 0x17 , 0xaf , 0x79)] interface ISpPhrase2 (ISpPhrase2Vtbl) : ISpPhrase (ISpPhraseVtbl) { fn GetXMLResult (ppszCoMemXMLResult : * mut LPWSTR , Options : SPXMLRESULTOPTIONS ,) -> HRESULT , fn GetXMLErrorInfo (pSemanticErrorInfo : * mut SPSEMANTICERRORINFO ,) -> HRESULT , fn GetAudio (ulStartElement : ULONG , cElements : ULONG , ppStream : * mut * mut ISpStreamFormat ,) -> HRESULT , } }
};
}
