// Generated macro for other_20192 (other)
macro_rules! Depcrate_um_bluetoothapisother_20192 {
() => {
// Module: crate::um::bluetoothapis
// Provides: {"other_20192"}
// Dependencies: {}
extern "system" { pub fn BluetoothFindFirstDevice (pbtsp : * const BLUETOOTH_DEVICE_SEARCH_PARAMS , pbtdi : * mut BLUETOOTH_DEVICE_INFO ,) -> HBLUETOOTH_DEVICE_FIND ; pub fn BluetoothFindNextDevice (hFind : HBLUETOOTH_DEVICE_FIND , pbtdi : * mut BLUETOOTH_DEVICE_INFO ,) -> BOOL ; pub fn BluetoothFindDeviceClose (hFind : HBLUETOOTH_DEVICE_FIND ,) -> BOOL ; pub fn BluetoothGetDeviceInfo (hRadio : HANDLE , pbtdi : * mut BLUETOOTH_DEVICE_INFO ,) -> DWORD ; pub fn BluetoothUpdateDeviceRecord (pbtdi : * const BLUETOOTH_DEVICE_INFO ,) -> DWORD ; pub fn BluetoothRemoveDevice (pAddress : * const BLUETOOTH_ADDRESS ,) -> DWORD ; }
};
}
