// Generated macro for IOCTL_IDE_PASS_THROUGH (const)
macro_rules! Depcrate_shared_ntddscsiIOCTL_IDE_PASS_THROUGH {
() => {
// Module: crate::shared::ntddscsi
// Provides: {"IOCTL_IDE_PASS_THROUGH"}
// Dependencies: {}
pub const IOCTL_IDE_PASS_THROUGH : ULONG = CTL_CODE ! (IOCTL_SCSI_BASE , 0x040a , METHOD_BUFFERED , FILE_READ_ACCESS | FILE_WRITE_ACCESS) ;
};
}
