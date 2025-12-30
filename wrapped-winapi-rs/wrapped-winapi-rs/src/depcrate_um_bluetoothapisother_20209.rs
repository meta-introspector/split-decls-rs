// Generated macro for other_20209 (other)
macro_rules! Depcrate_um_bluetoothapisother_20209 {
() => {
// Module: crate::um::bluetoothapis
// Provides: {"other_20209"}
// Dependencies: {}
extern "system" { pub fn BluetoothSetServiceState (hRadio : HANDLE , pbtdi : * const BLUETOOTH_DEVICE_INFO , pGuidService : * const GUID , dwServiceFlags : DWORD ,) -> DWORD ; pub fn BluetoothEnumerateInstalledServices (hRadio : HANDLE , pbtdi : * const BLUETOOTH_DEVICE_INFO , pcServiceInout : * mut DWORD , pGuidServices : * mut GUID ,) -> DWORD ; pub fn BluetoothEnableDiscovery (hRadio : HANDLE , fEnabled : BOOL ,) -> BOOL ; pub fn BluetoothIsDiscoverable (hRadio : HANDLE ,) -> BOOL ; pub fn BluetoothEnableIncomingConnections (hRadio : HANDLE , fEnabled : BOOL ,) -> BOOL ; pub fn BluetoothIsConnectable (hRadio : HANDLE ,) -> BOOL ; }
};
}
