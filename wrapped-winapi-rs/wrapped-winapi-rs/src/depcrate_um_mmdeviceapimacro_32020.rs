// Generated macro for macro_32020 (macro)
macro_rules! Depcrate_um_mmdeviceapimacro_32020 {
() => {
// Module: crate::um::mmdeviceapi
// Provides: {"macro_32020"}
// Dependencies: {}
RIDL ! { # [uuid (0x7991eec9 , 0x7e89 , 0x4d85 , 0x83 , 0x90 , 0x6c , 0x70 , 0x3c , 0xec , 0x60 , 0xc0)] interface IMMNotificationClient (IMMNotificationClientVtbl) : IUnknown (IUnknownVtbl) { fn OnDeviceStateChanged (pwstrDeviceId : LPCWSTR , dwNewState : DWORD ,) -> HRESULT , fn OnDeviceAdded (pwstrDeviceId : LPCWSTR ,) -> HRESULT , fn OnDeviceRemoved (pwstrDeviceId : LPCWSTR ,) -> HRESULT , fn OnDefaultDeviceChanged (flow : EDataFlow , role : ERole , pwstrDefaultDeviceId : LPCWSTR ,) -> HRESULT , fn OnPropertyValueChanged (pwstrDeviceId : LPCWSTR , key : PROPERTYKEY ,) -> HRESULT , } }
};
}
