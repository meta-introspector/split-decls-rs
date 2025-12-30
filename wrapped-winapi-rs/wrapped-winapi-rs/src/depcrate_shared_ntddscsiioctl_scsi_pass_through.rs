// Generated macro for IOCTL_SCSI_PASS_THROUGH (const)
macro_rules! Depcrate_shared_ntddscsiIOCTL_SCSI_PASS_THROUGH {
() => {
// Module: crate::shared::ntddscsi
// Provides: {"IOCTL_SCSI_PASS_THROUGH"}
// Dependencies: {}
pub const IOCTL_SCSI_PASS_THROUGH : ULONG = CTL_CODE ! (IOCTL_SCSI_BASE , 0x0401 , METHOD_BUFFERED , FILE_READ_ACCESS | FILE_WRITE_ACCESS) ;
};
}
