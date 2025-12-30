// Generated macro for macro_10684 (macro)
macro_rules! Depcrate_shared_usbmacro_10684 {
() => {
// Module: crate::shared::usb
// Provides: {"macro_10684"}
// Dependencies: {}
STRUCT ! { struct URB_BULK_OR_INTERRUPT_TRANSFER { Hdr : URB_HEADER , PipeHandle : USBD_PIPE_HANDLE , TransferFlags : ULONG , TransferBufferLength : ULONG , TransferBuffer : PVOID , TransferBufferMDL : PMDL , UrbLink : * mut URB , hca : URB_HCD_AREA , } }
};
}
