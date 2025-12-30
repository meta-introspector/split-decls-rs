// Generated macro for macro_36436 (macro)
macro_rules! Depcrate_um_sapi51macro_36436 {
() => {
// Module: crate::um::sapi51
// Provides: {"macro_36436"}
// Dependencies: {}
RIDL ! { # [uuid (0x678a932c , 0xea71 , 0x4446 , 0x9b , 0x41 , 0x78 , 0xfd , 0xa6 , 0x28 , 0x0a , 0x29)] interface ISpStreamFormatConverter (ISpStreamFormatConverterVtbl) : ISpStreamFormat (ISpStreamFormatVtbl) { fn SetBaseStream (pStream : * mut ISpStreamFormat , fSetFormatToBaseStreamFormat : BOOL , fWriteToBaseStream : BOOL ,) -> HRESULT , fn GetBaseStream (ppStream : * mut * mut ISpStreamFormat ,) -> HRESULT , fn SetFormat (rguidFormatIdOfConvertedStream : REFGUID , pWaveFormatExOfConvertedStream : * const WAVEFORMATEX ,) -> HRESULT , fn ResetSeekPosition () -> HRESULT , fn ScaleConvertedToBaseOffset (ullOffsetConvertedStream : ULONGLONG , pullOffsetBaseStream : * mut ULONGLONG ,) -> HRESULT , fn ScaleBaseToConvertedOffset (ullOffsetBaseStream : ULONGLONG , pullOffsetConvertedStream : * mut ULONGLONG ,) -> HRESULT , } }
};
}
