// Generated macro for IOCTL_ATA_PASS_THROUGH (const)
macro_rules! Depcrate_shared_ntddscsiIOCTL_ATA_PASS_THROUGH {
() => {
// Module: crate::shared::ntddscsi
// Provides: {"IOCTL_ATA_PASS_THROUGH"}
// Dependencies: {}
pub const IOCTL_ATA_PASS_THROUGH : ULONG = CTL_CODE ! (IOCTL_SCSI_BASE , 0x040b , METHOD_BUFFERED , FILE_READ_ACCESS | FILE_WRITE_ACCESS) ;
};
}
