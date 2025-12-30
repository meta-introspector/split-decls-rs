// Generated macro for macro_36435 (macro)
macro_rules! Depcrate_um_sapi51macro_36435 {
() => {
// Module: crate::um::sapi51
// Provides: {"macro_36435"}
// Dependencies: {}
RIDL ! { # [uuid (0x12e3cca9 , 0x7518 , 0x44c5 , 0xa5 , 0xe7 , 0xba , 0x5a , 0x79 , 0xcb , 0x92 , 0x9e)] interface ISpStream (ISpStreamVtbl) : ISpStreamFormat (ISpStreamFormatVtbl) { fn SetBaseStream (pStream : * mut IStream , rguidFormat : REFGUID , pWaveFormatEx : * const WAVEFORMATEX ,) -> HRESULT , fn GetBaseStream (ppStream : * mut * mut IStream ,) -> HRESULT , fn BindToFile (pszFileName : LPCWSTR , eMode : SPFILEMODE , pFormatId : * const GUID , pWaveFormatEx : * const WAVEFORMATEX , ullEventInterest : ULONGLONG ,) -> HRESULT , fn Close () -> HRESULT , } }
};
}
