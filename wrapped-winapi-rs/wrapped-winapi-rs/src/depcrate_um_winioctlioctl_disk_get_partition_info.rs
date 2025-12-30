// Generated macro for IOCTL_DISK_GET_PARTITION_INFO (const)
macro_rules! Depcrate_um_winioctlIOCTL_DISK_GET_PARTITION_INFO {
() => {
// Module: crate::um::winioctl
// Provides: {"IOCTL_DISK_GET_PARTITION_INFO"}
// Dependencies: {}
pub const IOCTL_DISK_GET_PARTITION_INFO : DWORD = CTL_CODE ! (IOCTL_DISK_BASE , 0x0001 , METHOD_BUFFERED , FILE_READ_ACCESS) ;
};
}
