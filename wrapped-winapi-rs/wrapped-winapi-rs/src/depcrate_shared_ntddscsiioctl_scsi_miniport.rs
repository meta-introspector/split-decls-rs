// Generated macro for IOCTL_SCSI_MINIPORT (const)
macro_rules! Depcrate_shared_ntddscsiIOCTL_SCSI_MINIPORT {
() => {
// Module: crate::shared::ntddscsi
// Provides: {"IOCTL_SCSI_MINIPORT"}
// Dependencies: {}
pub const IOCTL_SCSI_MINIPORT : ULONG = CTL_CODE ! (IOCTL_SCSI_BASE , 0x0402 , METHOD_BUFFERED , FILE_READ_ACCESS | FILE_WRITE_ACCESS) ;
};
}
