// Generated macro for IOCTL_MPIO_PASS_THROUGH_PATH_DIRECT (const)
macro_rules! Depcrate_shared_ntddscsiIOCTL_MPIO_PASS_THROUGH_PATH_DIRECT {
() => {
// Module: crate::shared::ntddscsi
// Provides: {"IOCTL_MPIO_PASS_THROUGH_PATH_DIRECT"}
// Dependencies: {}
pub const IOCTL_MPIO_PASS_THROUGH_PATH_DIRECT : ULONG = CTL_CODE ! (IOCTL_SCSI_BASE , 0x0410 , METHOD_BUFFERED , FILE_READ_ACCESS | FILE_WRITE_ACCESS) ;
};
}
