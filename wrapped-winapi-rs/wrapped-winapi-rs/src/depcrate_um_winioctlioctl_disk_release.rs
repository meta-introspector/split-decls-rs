// Generated macro for IOCTL_DISK_RELEASE (const)
macro_rules! Depcrate_um_winioctlIOCTL_DISK_RELEASE {
() => {
// Module: crate::um::winioctl
// Provides: {"IOCTL_DISK_RELEASE"}
// Dependencies: {}
pub const IOCTL_DISK_RELEASE : DWORD = CTL_CODE ! (IOCTL_DISK_BASE , 0x0205 , METHOD_BUFFERED , FILE_READ_ACCESS) ;
};
}
