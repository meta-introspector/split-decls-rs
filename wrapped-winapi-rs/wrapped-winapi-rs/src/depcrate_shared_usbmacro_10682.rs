// Generated macro for macro_10682 (macro)
macro_rules! Depcrate_shared_usbmacro_10682 {
() => {
// Module: crate::shared::usb
// Provides: {"macro_10682"}
// Dependencies: {}
# [cfg (target_pointer_width = "64")] STRUCT ! { struct URB_CONTROL_TRANSFER_EX { Hdr : URB_HEADER , PipeHandle : USBD_PIPE_HANDLE , TransferFlags : ULONG , TransferBufferLength : ULONG , TransferBuffer : PVOID , TransferBufferMDL : PMDL , Timeout : ULONG , Pad : ULONG , hca : URB_HCD_AREA , SetupPacket : [UCHAR ; 8] , } }
};
}
