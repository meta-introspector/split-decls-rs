// Generated macro for IOCTL_ATA_PASS_THROUGH_DIRECT (const)
macro_rules! Depcrate_shared_ntddscsiIOCTL_ATA_PASS_THROUGH_DIRECT {
() => {
// Module: crate::shared::ntddscsi
// Provides: {"IOCTL_ATA_PASS_THROUGH_DIRECT"}
// Dependencies: {}
pub const IOCTL_ATA_PASS_THROUGH_DIRECT : ULONG = CTL_CODE ! (IOCTL_SCSI_BASE , 0x040c , METHOD_BUFFERED , FILE_READ_ACCESS | FILE_WRITE_ACCESS) ;
};
}
