// Generated macro for macro_27727 (macro)
macro_rules! Depcrate_um_dsoundmacro_27727 {
() => {
// Module: crate::um::dsound
// Provides: {"macro_27727"}
// Dependencies: {}
RIDL ! { # [uuid (0x279afa83 , 0x4981 , 0x11ce , 0xa5 , 0x21 , 0x00 , 0x20 , 0xaf , 0x0b , 0xe5 , 0x60)] interface IDirectSound (IDirectSoundVtbl) : IUnknown (IUnknownVtbl) { fn CreateSoundBuffer (pcDSBufferDesc : LPCDSBUFFERDESC , ppDSBuffer : * mut LPDIRECTSOUNDBUFFER , pUnkOuter : LPUNKNOWN ,) -> HRESULT , fn GetCaps (pDSCaps : LPDSCAPS ,) -> HRESULT , fn DuplicateSoundBuffer (pDSBufferOriginal : LPDIRECTSOUNDBUFFER , ppDSBufferDuplicate : * mut LPDIRECTSOUNDBUFFER ,) -> HRESULT , fn SetCooperativeLevel (hWnd : HWND , dwLevel : DWORD ,) -> HRESULT , fn Compact () -> HRESULT , fn GetSpeakerConfig (pdwSpeakerConfig : LPDWORD ,) -> HRESULT , fn SetSpeakerConfig (dwSpeakerConfig : DWORD ,) -> HRESULT , fn Initialize (pcGuidDevice : LPCGUID ,) -> HRESULT , } }
};
}
