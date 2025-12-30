// Generated macro for macro_19842 (macro)
macro_rules! Depcrate_um_audioclientmacro_19842 {
() => {
// Module: crate::um::audioclient
// Provides: {"macro_19842"}
// Dependencies: {}
RIDL ! { # [uuid (0x93014887 , 0x242d , 0x4068 , 0x8a , 0x15 , 0xcf , 0x5e , 0x93 , 0xb9 , 0x0f , 0xe3)] interface IAudioStreamVolume (IAudioStreamVolumeVtbl) : IUnknown (IUnknownVtbl) { fn GetChannelCount (pdwCount : * mut UINT32 ,) -> HRESULT , fn SetChannelVolume (dwIndex : UINT32 , fLevel : c_float ,) -> HRESULT , fn GetChannelVolume (dwIndex : UINT32 , pfLevel : * mut c_float ,) -> HRESULT , fn SetAllVolumes (dwCount : UINT32 , pfVolumes : * const c_float ,) -> HRESULT , fn GetAllVolumes (dwCount : UINT32 , pfVolumes : * mut c_float ,) -> HRESULT , } }
};
}
