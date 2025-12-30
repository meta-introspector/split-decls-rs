// Generated macro for IOCTL_DISK_SET_CACHE_INFORMATION (const)
macro_rules! Depcrate_um_winioctlIOCTL_DISK_SET_CACHE_INFORMATION {
() => {
// Module: crate::um::winioctl
// Provides: {"IOCTL_DISK_SET_CACHE_INFORMATION"}
// Dependencies: {}
pub const IOCTL_DISK_SET_CACHE_INFORMATION : DWORD = CTL_CODE ! (IOCTL_DISK_BASE , 0x0036 , METHOD_BUFFERED , FILE_READ_ACCESS | FILE_WRITE_ACCESS) ;
};
}
