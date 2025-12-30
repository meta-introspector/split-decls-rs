// Generated macro for macro_28508 (macro)
macro_rules! Depcrate_um_endpointvolumemacro_28508 {
() => {
// Module: crate::um::endpointvolume
// Provides: {"macro_28508"}
// Dependencies: {}
RIDL ! { # [uuid (0xc02216f6 , 0x8c67 , 0x4b5b , 0x9d , 0x00 , 0xd0 , 0x08 , 0xe7 , 0x3e , 0x00 , 0x64)] interface IAudioMeterInformation (IAudioMeterInformationVtbl) : IUnknown (IUnknownVtbl) { fn GetPeakValue (pfPeak : * mut c_float ,) -> HRESULT , fn GetMeteringChannelCount (pnChannelCount : * mut UINT ,) -> HRESULT , fn GetChannelsPeakValues (u32ChannelCount : UINT32 , afPeakValues : * mut c_float ,) -> HRESULT , fn QueryHardwareSupport (pdwHardwareSupportMask : * mut DWORD ,) -> HRESULT , } }
};
}
