// Generated macro for macro_10683 (macro)
macro_rules! Depcrate_shared_usbmacro_10683 {
() => {
// Module: crate::shared::usb
// Provides: {"macro_10683"}
// Dependencies: {}
# [cfg (target_pointer_width = "32")] STRUCT ! { struct URB_CONTROL_TRANSFER_EX { Hdr : URB_HEADER , PipeHandle : USBD_PIPE_HANDLE , TransferFlags : ULONG , TransferBufferLength : ULONG , TransferBuffer : PVOID , TransferBufferMDL : PMDL , Timeout : ULONG , hca : URB_HCD_AREA , SetupPacket : [UCHAR ; 8] , } }
};
}
