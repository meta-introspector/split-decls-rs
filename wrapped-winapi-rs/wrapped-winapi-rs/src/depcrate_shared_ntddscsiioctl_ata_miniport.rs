// Generated macro for IOCTL_ATA_MINIPORT (const)
macro_rules! Depcrate_shared_ntddscsiIOCTL_ATA_MINIPORT {
() => {
// Module: crate::shared::ntddscsi
// Provides: {"IOCTL_ATA_MINIPORT"}
// Dependencies: {}
pub const IOCTL_ATA_MINIPORT : ULONG = CTL_CODE ! (IOCTL_SCSI_BASE , 0x040d , METHOD_BUFFERED , FILE_READ_ACCESS | FILE_WRITE_ACCESS) ;
};
}
