// Generated macro for IOCTL_DISK_GET_WRITE_CACHE_STATE (const)
macro_rules! Depcrate_um_winioctlIOCTL_DISK_GET_WRITE_CACHE_STATE {
() => {
// Module: crate::um::winioctl
// Provides: {"IOCTL_DISK_GET_WRITE_CACHE_STATE"}
// Dependencies: {}
pub const IOCTL_DISK_GET_WRITE_CACHE_STATE : DWORD = CTL_CODE ! (IOCTL_DISK_BASE , 0x0037 , METHOD_BUFFERED , FILE_READ_ACCESS) ;
};
}
