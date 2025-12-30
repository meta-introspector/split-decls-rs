// Generated macro for macro_19838 (macro)
macro_rules! Depcrate_um_audioclientmacro_19838 {
() => {
// Module: crate::um::audioclient
// Provides: {"macro_19838"}
// Dependencies: {}
RIDL ! { # [uuid (0x1cb9ad4c , 0xdbfa , 0x4c32 , 0xb1 , 0x78 , 0xc2 , 0xf5 , 0x68 , 0xa7 , 0x03 , 0xb2)] interface IAudioClient (IAudioClientVtbl) : IUnknown (IUnknownVtbl) { fn Initialize (ShareMode : AUDCLNT_SHAREMODE , StreamFlags : DWORD , hnsBufferDuration : REFERENCE_TIME , hnsPeriodicity : REFERENCE_TIME , pFormat : * const WAVEFORMATEX , AudioSessionGuid : LPCGUID ,) -> HRESULT , fn GetBufferSize (pNumBufferFrames : * mut UINT32 ,) -> HRESULT , fn GetStreamLatency (phnsLatency : * mut REFERENCE_TIME ,) -> HRESULT , fn GetCurrentPadding (pNumPaddingFrames : * mut UINT32 ,) -> HRESULT , fn IsFormatSupported (ShareMode : AUDCLNT_SHAREMODE , pFormat : * const WAVEFORMATEX , ppClosestMatch : * mut * mut WAVEFORMATEX ,) -> HRESULT , fn GetMixFormat (ppDeviceFormat : * mut * mut WAVEFORMATEX ,) -> HRESULT , fn GetDevicePeriod (phnsDefaultDevicePeriod : * mut REFERENCE_TIME , phnsMinimumDevicePeriod : * mut REFERENCE_TIME ,) -> HRESULT , fn Start () -> HRESULT , fn Stop () -> HRESULT , fn Reset () -> HRESULT , fn SetEventHandle (eventHandle : HANDLE ,) -> HRESULT , fn GetService (riid : REFIID , ppv : * mut LPVOID ,) -> HRESULT , } }
};
}
