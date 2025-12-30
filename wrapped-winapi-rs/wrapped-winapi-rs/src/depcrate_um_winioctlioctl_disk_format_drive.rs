// Generated macro for IOCTL_DISK_FORMAT_DRIVE (const)
macro_rules! Depcrate_um_winioctlIOCTL_DISK_FORMAT_DRIVE {
() => {
// Module: crate::um::winioctl
// Provides: {"IOCTL_DISK_FORMAT_DRIVE"}
// Dependencies: {}
pub const IOCTL_DISK_FORMAT_DRIVE : DWORD = CTL_CODE ! (IOCTL_DISK_BASE , 0x00f3 , METHOD_BUFFERED , FILE_READ_ACCESS | FILE_WRITE_ACCESS) ;
};
}
