// Generated macro for IOCTL_DISK_CHECK_VERIFY (const)
macro_rules! Depcrate_um_winioctlIOCTL_DISK_CHECK_VERIFY {
() => {
// Module: crate::um::winioctl
// Provides: {"IOCTL_DISK_CHECK_VERIFY"}
// Dependencies: {}
pub const IOCTL_DISK_CHECK_VERIFY : DWORD = CTL_CODE ! (IOCTL_DISK_BASE , 0x0200 , METHOD_BUFFERED , FILE_READ_ACCESS) ;
};
}
