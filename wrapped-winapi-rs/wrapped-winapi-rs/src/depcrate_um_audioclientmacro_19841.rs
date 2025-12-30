// Generated macro for macro_19841 (macro)
macro_rules! Depcrate_um_audioclientmacro_19841 {
() => {
// Module: crate::um::audioclient
// Provides: {"macro_19841"}
// Dependencies: {}
RIDL ! { # [uuid (0xcd63314f , 0x3fba , 0x4a1b , 0x81 , 0x2c , 0xef , 0x96 , 0x35 , 0x87 , 0x28 , 0xe7)] interface IAudioClock (IAudioClockVtbl) : IUnknown (IUnknownVtbl) { fn GetFrequency (pu64Frequency : * mut UINT64 ,) -> HRESULT , fn GetPosition (pu64Position : * mut UINT64 , pu64QPCPosition : * mut UINT64 ,) -> HRESULT , fn GetCharacteristics (pdwCharacteristics : * mut DWORD ,) -> HRESULT , } }
};
}
