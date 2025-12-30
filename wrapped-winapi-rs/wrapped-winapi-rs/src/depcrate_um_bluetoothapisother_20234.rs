// Generated macro for other_20234 (other)
macro_rules! Depcrate_um_bluetoothapisother_20234 {
() => {
// Module: crate::um::bluetoothapis
// Provides: {"other_20234"}
// Dependencies: {}
extern "system" { pub fn BluetoothSdpEnumAttributes (pSDPStream : LPBYTE , cbStreamSize : ULONG , pfnCallback : PFN_BLUETOOTH_ENUM_ATTRIBUTES_CALLBACK , pvParam : LPVOID ,) -> BOOL ; pub fn BluetoothSetLocalServiceInfo (hRadioIn : HANDLE , pClassGuid : * const GUID , ulInstance : ULONG , pServiceInfoIn : * const BLUETOOTH_LOCAL_SERVICE_INFO ,) -> DWORD ; pub fn BluetoothIsVersionAvailable (MajorVersion : UCHAR , MinorVersion : UCHAR ,) -> BOOL ; }
};
}
