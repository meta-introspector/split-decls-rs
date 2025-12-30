// Generated macro for IOCTL_SCSI_PASS_THROUGH_EX (const)
macro_rules! Depcrate_shared_ntddscsiIOCTL_SCSI_PASS_THROUGH_EX {
() => {
// Module: crate::shared::ntddscsi
// Provides: {"IOCTL_SCSI_PASS_THROUGH_EX"}
// Dependencies: {}
pub const IOCTL_SCSI_PASS_THROUGH_EX : ULONG = CTL_CODE ! (IOCTL_SCSI_BASE , 0x0411 , METHOD_BUFFERED , FILE_READ_ACCESS | FILE_WRITE_ACCESS) ;
};
}
