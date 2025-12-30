// Generated macro for macro_10666 (macro)
macro_rules! Depcrate_shared_usbmacro_10666 {
() => {
// Module: crate::shared::usb
// Provides: {"macro_10666"}
// Dependencies: {}
STRUCT ! { struct URB_CONTROL_GET_STATUS_REQUEST { Hdr : URB_HEADER , Reserved : PVOID , Reserved0 : ULONG , TransferBufferLength : ULONG , TransferBuffer : PVOID , TransferBufferMDL : PMDL , UrbLink : * mut URB , hca : URB_HCD_AREA , Reserved1 : [UCHAR ; 4] , Index : USHORT , Reserved2 : USHORT , } }
};
}
