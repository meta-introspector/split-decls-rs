// Generated macro for IOCTL_MPIO_PASS_THROUGH_PATH (const)
macro_rules! Depcrate_shared_ntddscsiIOCTL_MPIO_PASS_THROUGH_PATH {
() => {
// Module: crate::shared::ntddscsi
// Provides: {"IOCTL_MPIO_PASS_THROUGH_PATH"}
// Dependencies: {}
pub const IOCTL_MPIO_PASS_THROUGH_PATH : ULONG = CTL_CODE ! (IOCTL_SCSI_BASE , 0x040f , METHOD_BUFFERED , FILE_READ_ACCESS | FILE_WRITE_ACCESS) ;
};
}
