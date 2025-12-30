// Generated macro for other_20212 (other)
macro_rules! Depcrate_um_bluetoothapisother_20212 {
() => {
// Module: crate::um::bluetoothapis
// Provides: {"other_20212"}
// Dependencies: {}
extern "system" { pub fn BluetoothRegisterForAuthentication (pbtdi : * const BLUETOOTH_DEVICE_INFO , phRegHandle : * mut HBLUETOOTH_AUTHENTICATION_REGISTRATION , pfnCallback : PFN_AUTHENTICATION_CALLBACK , pvParam : PVOID ,) -> DWORD ; }
};
}
