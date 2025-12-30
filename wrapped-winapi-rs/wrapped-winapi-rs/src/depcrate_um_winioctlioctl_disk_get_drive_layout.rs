// Generated macro for IOCTL_DISK_GET_DRIVE_LAYOUT (const)
macro_rules! Depcrate_um_winioctlIOCTL_DISK_GET_DRIVE_LAYOUT {
() => {
// Module: crate::um::winioctl
// Provides: {"IOCTL_DISK_GET_DRIVE_LAYOUT"}
// Dependencies: {}
pub const IOCTL_DISK_GET_DRIVE_LAYOUT : DWORD = CTL_CODE ! (IOCTL_DISK_BASE , 0x0003 , METHOD_BUFFERED , FILE_READ_ACCESS) ;
};
}
