// Generated macro for other_20177 (other)
macro_rules! Depcrate_um_bluetoothapisother_20177 {
() => {
// Module: crate::um::bluetoothapis
// Provides: {"other_20177"}
// Dependencies: {}
extern "system" { pub fn BluetoothFindFirstRadio (pbtfrp : * const BLUETOOTH_FIND_RADIO_PARAMS , phRadio : * mut HANDLE ,) -> HBLUETOOTH_RADIO_FIND ; pub fn BluetoothFindNextRadio (hFind : HBLUETOOTH_RADIO_FIND , phRadio : * mut HANDLE ,) -> BOOL ; pub fn BluetoothFindRadioClose (hFind : HBLUETOOTH_RADIO_FIND ,) -> BOOL ; }
};
}
