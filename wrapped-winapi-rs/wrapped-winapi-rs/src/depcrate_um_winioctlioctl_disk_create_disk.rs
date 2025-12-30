// Generated macro for IOCTL_DISK_CREATE_DISK (const)
macro_rules! Depcrate_um_winioctlIOCTL_DISK_CREATE_DISK {
() => {
// Module: crate::um::winioctl
// Provides: {"IOCTL_DISK_CREATE_DISK"}
// Dependencies: {}
pub const IOCTL_DISK_CREATE_DISK : DWORD = CTL_CODE ! (IOCTL_DISK_BASE , 0x0016 , METHOD_BUFFERED , FILE_READ_ACCESS | FILE_WRITE_ACCESS) ;
};
}
