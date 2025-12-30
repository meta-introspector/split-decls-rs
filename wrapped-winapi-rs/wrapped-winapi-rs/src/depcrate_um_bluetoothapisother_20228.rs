// Generated macro for other_20228 (other)
macro_rules! Depcrate_um_bluetoothapisother_20228 {
() => {
// Module: crate::um::bluetoothapis
// Provides: {"other_20228"}
// Dependencies: {}
extern "system" { pub fn BluetoothSdpGetContainerElementData (pContainerStream : LPBYTE , cbContainerLength : ULONG , pElement : * mut HBLUETOOTH_CONTAINER_ELEMENT , pData : PSDP_ELEMENT_DATA ,) -> DWORD ; pub fn BluetoothSdpGetAttributeValue (pRecordStream : LPBYTE , cbRecordLength : ULONG , usAttributeId : USHORT , pAttributeData : PSDP_ELEMENT_DATA ,) -> DWORD ; }
};
}
