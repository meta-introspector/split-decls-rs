// Generated macro for IOCTL_DISK_RESERVE (const)
macro_rules! Depcrate_um_winioctlIOCTL_DISK_RESERVE {
() => {
// Module: crate::um::winioctl
// Provides: {"IOCTL_DISK_RESERVE"}
// Dependencies: {}
pub const IOCTL_DISK_RESERVE : DWORD = CTL_CODE ! (IOCTL_DISK_BASE , 0x0204 , METHOD_BUFFERED , FILE_READ_ACCESS) ;
};
}
