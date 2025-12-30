// Generated macro for other_20205 (other)
macro_rules! Depcrate_um_bluetoothapisother_20205 {
() => {
// Module: crate::um::bluetoothapis
// Provides: {"other_20205"}
// Dependencies: {}
extern "system" { pub fn BluetoothAuthenticateDeviceEx (hwndParentIn : HWND , hRadioIn : HANDLE , pbtdiInout : * mut BLUETOOTH_DEVICE_INFO , pbtOobData : PBLUETOOTH_OOB_DATA_INFO , authenticationRequirement : AUTHENTICATION_REQUIREMENTS ,) -> DWORD ; pub fn BluetoothAuthenticateMultipleDevices (hwndParent : HWND , hRadio : HANDLE , cDevices : DWORD , rgbtdi : * mut BLUETOOTH_DEVICE_INFO ,) -> DWORD ; }
};
}
