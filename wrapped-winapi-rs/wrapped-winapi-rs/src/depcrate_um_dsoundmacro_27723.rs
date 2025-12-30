// Generated macro for macro_27723 (macro)
macro_rules! Depcrate_um_dsoundmacro_27723 {
() => {
// Module: crate::um::dsound
// Provides: {"macro_27723"}
// Dependencies: {}
RIDL ! { # [uuid (0x279afa85 , 0x4981 , 0x11ce , 0xa5 , 0x21 , 0x00 , 0x20 , 0xaf , 0x0b , 0xe5 , 0x60)] interface IDirectSoundBuffer (IDirectSoundBufferVtbl) : IUnknown (IUnknownVtbl) { fn GetCaps (pDSBufferCaps : LPDSBCAPS ,) -> HRESULT , fn GetCurrentPosition (pdwCurrentPlayCursor : LPDWORD , pdwCurrentWriteCursor : LPDWORD ,) -> HRESULT , fn GetFormat (pwfxFormat : LPWAVEFORMATEX , dwSizeAllocated : DWORD , pdwSizeWritten : LPDWORD ,) -> HRESULT , fn GetVolume (plVolume : LPLONG ,) -> HRESULT , fn GetPan (plPan : LPLONG ,) -> HRESULT , fn GetFrequency (pdwFrequency : LPDWORD ,) -> HRESULT , fn GetStatus (pdwStatus : LPDWORD ,) -> HRESULT , fn Initialize (pDirectSound : LPDIRECTSOUND , pcDSBufferDesc : LPCDSBUFFERDESC ,) -> HRESULT , fn Lock (dwOffset : DWORD , dwBytes : DWORD , ppvAudioPtr1 : * mut LPVOID , pdwAudioBytes1 : LPDWORD , ppvAudioPtr2 : * mut LPVOID , pdwAudioBytes2 : LPDWORD , dwFlags : DWORD ,) -> HRESULT , fn Play (dwReserved1 : DWORD , dwPriority : DWORD , dwFlags : DWORD ,) -> HRESULT , fn SetCurrentPosition (dwNewPosition : DWORD ,) -> HRESULT , fn SetFormat (pcfxFormat : LPCWAVEFORMATEX ,) -> HRESULT , fn SetVolume (lVolume : LONG ,) -> HRESULT , fn SetPan (lPan : LONG ,) -> HRESULT , fn SetFrequency (dwFrequency : DWORD ,) -> HRESULT , fn Stop () -> HRESULT , fn Unlock (pvAudioPtr1 : LPVOID , dwAudioBytes1 : DWORD , pvAudioPtr2 : LPVOID , dwAudioBytes2 : DWORD ,) -> HRESULT , fn Restore () -> HRESULT , } }
};
}
