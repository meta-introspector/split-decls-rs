// Generated macro for macro_10687 (macro)
macro_rules! Depcrate_shared_usbmacro_10687 {
() => {
// Module: crate::shared::usb
// Provides: {"macro_10687"}
// Dependencies: {}
STRUCT ! { struct URB_ISOCH_TRANSFER { Hdr : URB_HEADER , PipeHandle : USBD_PIPE_HANDLE , TransferFlags : ULONG , TransferBufferLength : ULONG , TransferBuffer : PVOID , TransferBufferMDL : PMDL , UrbLink : * mut URB , hca : URB_HCD_AREA , StartFrame : ULONG , NumberOfPackets : ULONG , ErrorCount : ULONG , IsoPacket : [USBD_ISO_PACKET_DESCRIPTOR ; 1] , } }
};
}
