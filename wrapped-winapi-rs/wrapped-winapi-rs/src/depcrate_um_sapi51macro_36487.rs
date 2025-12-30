// Generated macro for macro_36487 (macro)
macro_rules! Depcrate_um_sapi51macro_36487 {
() => {
// Module: crate::um::sapi51
// Provides: {"macro_36487"}
// Dependencies: {}
RIDL ! { # [uuid (0x20b053be , 0xe235 , 0x43cd , 0x9a , 0x2a , 0x8d , 0x17 , 0xa4 , 0x8b , 0x78 , 0x42)] interface ISpRecoResult (ISpRecoResultVtbl) : ISpPhrase (ISpPhraseVtbl) { fn GetResultTimes (pTimes : * mut SPRECORESULTTIMES ,) -> HRESULT , fn GetAlternates (ulStartElement : ULONG , cElements : ULONG , ulRequestCount : ULONG , ppPhrases : * mut * mut ISpPhraseAlt , pcPhrasesReturned : * mut ULONG ,) -> HRESULT , fn GetAudio (ulStartElement : ULONG , cElements : ULONG , ppStream : * mut * mut ISpStreamFormat ,) -> HRESULT , fn SpeakAudio (ulStartElement : ULONG , cElements : ULONG , dwFlags : DWORD , pulStreamNumber : * mut ULONG ,) -> HRESULT , fn Serialize (ppCoMemSerializedResult : * mut * mut SPSERIALIZEDRESULT ,) -> HRESULT , fn ScaleAudio (pAudioFormatId : * const GUID , pWaveFormatEx : * const WAVEFORMATEX ,) -> HRESULT , fn GetRecoContext (ppRecoContext : * mut * mut ISpRecoContext ,) -> HRESULT , } }
};
}
