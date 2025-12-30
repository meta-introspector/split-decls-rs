// Generated macro for IOCTL_DISK_RESET_SNAPSHOT_INFO (const)
macro_rules! Depcrate_um_winioctlIOCTL_DISK_RESET_SNAPSHOT_INFO {
() => {
// Module: crate::um::winioctl
// Provides: {"IOCTL_DISK_RESET_SNAPSHOT_INFO"}
// Dependencies: {}
pub const IOCTL_DISK_RESET_SNAPSHOT_INFO : DWORD = CTL_CODE ! (IOCTL_DISK_BASE , 0x0084 , METHOD_BUFFERED , FILE_READ_ACCESS | FILE_WRITE_ACCESS) ;
};
}
