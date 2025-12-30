// Generated macro for IOCTL_MPIO_PASS_THROUGH_PATH_EX (const)
macro_rules! Depcrate_shared_ntddscsiIOCTL_MPIO_PASS_THROUGH_PATH_EX {
() => {
// Module: crate::shared::ntddscsi
// Provides: {"IOCTL_MPIO_PASS_THROUGH_PATH_EX"}
// Dependencies: {}
pub const IOCTL_MPIO_PASS_THROUGH_PATH_EX : ULONG = CTL_CODE ! (IOCTL_SCSI_BASE , 0x0413 , METHOD_BUFFERED , FILE_READ_ACCESS | FILE_WRITE_ACCESS) ;
};
}
