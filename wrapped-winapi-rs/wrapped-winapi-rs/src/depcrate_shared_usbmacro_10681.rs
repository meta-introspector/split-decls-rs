// Generated macro for macro_10681 (macro)
macro_rules! Depcrate_shared_usbmacro_10681 {
() => {
// Module: crate::shared::usb
// Provides: {"macro_10681"}
// Dependencies: {}
STRUCT ! { struct URB_CONTROL_TRANSFER { Hdr : URB_HEADER , PipeHandle : USBD_PIPE_HANDLE , TransferFlags : ULONG , TransferBufferLength : ULONG , TransferBuffer : PVOID , TransferBufferMDL : PMDL , UrbLink : * mut URB , hca : URB_HCD_AREA , SetupPacket : [UCHAR ; 8] , } }
};
}
