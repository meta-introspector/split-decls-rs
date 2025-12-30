// Generated macro for IOCTL_DISK_SET_DISK_ATTRIBUTES (const)
macro_rules! Depcrate_um_winioctlIOCTL_DISK_SET_DISK_ATTRIBUTES {
() => {
// Module: crate::um::winioctl
// Provides: {"IOCTL_DISK_SET_DISK_ATTRIBUTES"}
// Dependencies: {}
pub const IOCTL_DISK_SET_DISK_ATTRIBUTES : DWORD = CTL_CODE ! (IOCTL_DISK_BASE , 0x003d , METHOD_BUFFERED , FILE_READ_ACCESS | FILE_WRITE_ACCESS) ;
};
}
