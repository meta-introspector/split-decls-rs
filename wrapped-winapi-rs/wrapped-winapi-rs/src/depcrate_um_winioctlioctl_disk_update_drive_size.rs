// Generated macro for IOCTL_DISK_UPDATE_DRIVE_SIZE (const)
macro_rules! Depcrate_um_winioctlIOCTL_DISK_UPDATE_DRIVE_SIZE {
() => {
// Module: crate::um::winioctl
// Provides: {"IOCTL_DISK_UPDATE_DRIVE_SIZE"}
// Dependencies: {}
pub const IOCTL_DISK_UPDATE_DRIVE_SIZE : DWORD = CTL_CODE ! (IOCTL_DISK_BASE , 0x0032 , METHOD_BUFFERED , FILE_READ_ACCESS | FILE_WRITE_ACCESS) ;
};
}
