// Generated macro for macro_10668 (macro)
macro_rules! Depcrate_shared_usbmacro_10668 {
() => {
// Module: crate::shared::usb
// Provides: {"macro_10668"}
// Dependencies: {}
STRUCT ! { struct URB_CONTROL_VENDOR_OR_CLASS_REQUEST { Hdr : URB_HEADER , Reserved : PVOID , TransferFlags : ULONG , TransferBufferLength : ULONG , TransferBuffer : PVOID , TransferBufferMDL : PMDL , UrbLink : * mut URB , hca : URB_HCD_AREA , RequestTypeReservedBits : UCHAR , Request : UCHAR , Value : USHORT , Index : USHORT , Reserved1 : USHORT , } }
};
}
