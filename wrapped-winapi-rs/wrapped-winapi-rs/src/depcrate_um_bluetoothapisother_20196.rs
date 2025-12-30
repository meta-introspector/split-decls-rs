// Generated macro for other_20196 (other)
macro_rules! Depcrate_um_bluetoothapisother_20196 {
() => {
// Module: crate::um::bluetoothapis
// Provides: {"other_20196"}
// Dependencies: {}
extern "system" { pub fn BluetoothSelectDevices (pbtsdp : * mut BLUETOOTH_SELECT_DEVICE_PARAMS ,) -> BOOL ; pub fn BluetoothSelectDevicesFree (pbtsdp : * mut BLUETOOTH_SELECT_DEVICE_PARAMS ,) -> BOOL ; pub fn BluetoothDisplayDeviceProperties (hwndParent : HWND , pbtdi : * mut BLUETOOTH_DEVICE_INFO ,) -> BOOL ; pub fn BluetoothAuthenticateDevice (hwndParent : HWND , hRadio : HANDLE , pbtbi : * mut BLUETOOTH_DEVICE_INFO , pszPasskey : PWSTR , ulPasskeyLength : ULONG ,) -> DWORD ; }
};
}
