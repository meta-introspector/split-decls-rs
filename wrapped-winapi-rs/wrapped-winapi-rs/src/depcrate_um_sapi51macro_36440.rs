// Generated macro for macro_36440 (macro)
macro_rules! Depcrate_um_sapi51macro_36440 {
() => {
// Module: crate::um::sapi51
// Provides: {"macro_36440"}
// Dependencies: {}
RIDL ! { # [uuid (0xc05c768f , 0xfae8 , 0x4ec2 , 0x8e , 0x07 , 0x33 , 0x83 , 0x21 , 0xc1 , 0x24 , 0x52)] interface ISpAudio (ISpAudioVtbl) : ISpStreamFormat (ISpStreamFormatVtbl) { fn SetState (NewState : SPAUDIOSTATE , ullReserved : ULONGLONG ,) -> HRESULT , fn SetFormat (rguidFmtId : REFGUID , pWaveFormatEx : * const WAVEFORMATEX ,) -> HRESULT , fn GetStatus (pStatus : * mut SPAUDIOSTATUS ,) -> HRESULT , fn SetBufferInfo (pBuffInfo : * const SPAUDIOBUFFERINFO ,) -> HRESULT , fn GetBufferInfo (pBuffInfo : * mut SPAUDIOBUFFERINFO ,) -> HRESULT , fn GetDefaultFormat (pFormatId : * mut GUID , ppCoMemWaveFormatEx : * mut * mut WAVEFORMATEX ,) -> HRESULT , fn EventHandle () -> HANDLE , fn GetVolumeLevel (pLevel : * mut ULONG ,) -> HRESULT , fn SetVolumeLevel (Level : ULONG ,) -> HRESULT , fn GetBufferNotifySize (pcbSize : * mut ULONG ,) -> HRESULT , fn SetBufferNotifySize (cbSize : ULONG ,) -> HRESULT , } }
};
}
