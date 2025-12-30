// Generated macro for IOCTL_MINIPORT_PROCESS_SERVICE_IRP (const)
macro_rules! Depcrate_shared_ntddscsiIOCTL_MINIPORT_PROCESS_SERVICE_IRP {
() => {
// Module: crate::shared::ntddscsi
// Provides: {"IOCTL_MINIPORT_PROCESS_SERVICE_IRP"}
// Dependencies: {}
pub const IOCTL_MINIPORT_PROCESS_SERVICE_IRP : ULONG = CTL_CODE ! (IOCTL_SCSI_BASE , 0x040e , METHOD_BUFFERED , FILE_READ_ACCESS | FILE_WRITE_ACCESS) ;
};
}
