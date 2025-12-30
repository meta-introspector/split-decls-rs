// Generated macro for macro_32024 (macro)
macro_rules! Depcrate_um_mmdeviceapimacro_32024 {
() => {
// Module: crate::um::mmdeviceapi
// Provides: {"macro_32024"}
// Dependencies: {}
RIDL ! { # [uuid (0xa95664d2 , 0x9614 , 0x4f35 , 0xa7 , 0x46 , 0xde , 0x8d , 0xb6 , 0x36 , 0x17 , 0xe6)] interface IMMDeviceEnumerator (IMMDeviceEnumeratorVtbl) : IUnknown (IUnknownVtbl) { fn EnumAudioEndpoints (dataFlow : EDataFlow , dwStateMask : DWORD , ppDevices : * mut * mut IMMDeviceCollection ,) -> HRESULT , fn GetDefaultAudioEndpoint (dataFlow : EDataFlow , role : ERole , ppEndpoint : * mut * mut IMMDevice ,) -> HRESULT , fn GetDevice (pwstrId : LPCWSTR , ppDevices : * mut * mut IMMDevice ,) -> HRESULT , fn RegisterEndpointNotificationCallback (pClient : * mut IMMNotificationClient ,) -> HRESULT , fn UnregisterEndpointNotificationCallback (pClient : * mut IMMNotificationClient ,) -> HRESULT , } }
};
}
