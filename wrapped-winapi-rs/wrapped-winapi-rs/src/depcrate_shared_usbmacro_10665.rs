// Generated macro for macro_10665 (macro)
macro_rules! Depcrate_shared_usbmacro_10665 {
() => {
// Module: crate::shared::usb
// Provides: {"macro_10665"}
// Dependencies: {}
STRUCT ! { struct URB_CONTROL_DESCRIPTOR_REQUEST { Hdr : URB_HEADER , Reserved : PVOID , Reserved0 : ULONG , TransferBufferLength : ULONG , TransferBuffer : PVOID , TransferBufferMDL : PMDL , UrbLink : * mut URB , hca : URB_HCD_AREA , Reserved1 : USHORT , Index : UCHAR , DescriptorType : UCHAR , LanguageId : USHORT , Reserved2 : USHORT , } }
};
}
