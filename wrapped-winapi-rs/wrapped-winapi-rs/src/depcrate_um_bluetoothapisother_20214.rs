// Generated macro for other_20214 (other)
macro_rules! Depcrate_um_bluetoothapisother_20214 {
() => {
// Module: crate::um::bluetoothapis
// Provides: {"other_20214"}
// Dependencies: {}
extern "system" { pub fn BluetoothRegisterForAuthenticationEx (pbtdiIn : * const BLUETOOTH_DEVICE_INFO , phRegHandleOut : * mut HBLUETOOTH_AUTHENTICATION_REGISTRATION , pfnCallbackIn : PFN_AUTHENTICATION_CALLBACK_EX , pvParam : PVOID ,) -> DWORD ; pub fn BluetoothUnregisterAuthentication (hRegHandle : HBLUETOOTH_AUTHENTICATION_REGISTRATION ,) -> BOOL ; pub fn BluetoothSendAuthenticationResponse (hRadio : HANDLE , pbtdi : * const BLUETOOTH_DEVICE_INFO , pszPasskey : LPCWSTR ,) -> DWORD ; }
};
}
