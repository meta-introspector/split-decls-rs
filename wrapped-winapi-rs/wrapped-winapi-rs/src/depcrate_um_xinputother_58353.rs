// Generated macro for other_58353 (other)
macro_rules! Depcrate_um_xinputother_58353 {
() => {
// Module: crate::um::xinput
// Provides: {"other_58353"}
// Dependencies: {}
extern "system" { pub fn XInputGetState (dwUserIndex : DWORD , pState : * mut XINPUT_STATE ,) -> DWORD ; pub fn XInputSetState (dwUserIndex : DWORD , pVibration : * mut XINPUT_VIBRATION ,) -> DWORD ; pub fn XInputGetCapabilities (dwUserIndex : DWORD , dwFlags : DWORD , pCapabilities : * mut XINPUT_CAPABILITIES ,) -> DWORD ; pub fn XInputEnable (enable : BOOL ,) ; pub fn XInputGetAudioDeviceIds (dwUserIndex : DWORD , pRenderDeviceId : LPWSTR , pRenderCount : * mut UINT , pCaptureDeviceId : LPWSTR , pCaptureCount : * mut UINT ,) -> DWORD ; pub fn XInputGetBatteryInformation (dwUserIndex : DWORD , devType : BYTE , pBatteryInformation : * mut XINPUT_BATTERY_INFORMATION ,) -> DWORD ; pub fn XInputGetKeystroke (dwUserIndex : DWORD , dwReserved : DWORD , pKeystroke : PXINPUT_KEYSTROKE ,) -> DWORD ; pub fn XInputGetDSoundAudioDeviceGuids (dwUserIndex : DWORD , pDSoundRenderGuid : * mut GUID , pDSoundCaptureGuid : * mut GUID ,) -> DWORD ; }
};
}
