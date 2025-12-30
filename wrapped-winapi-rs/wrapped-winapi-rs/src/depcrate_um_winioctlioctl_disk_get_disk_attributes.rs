// Generated macro for IOCTL_DISK_GET_DISK_ATTRIBUTES (const)
macro_rules! Depcrate_um_winioctlIOCTL_DISK_GET_DISK_ATTRIBUTES {
() => {
// Module: crate::um::winioctl
// Provides: {"IOCTL_DISK_GET_DISK_ATTRIBUTES"}
// Dependencies: {}
pub const IOCTL_DISK_GET_DISK_ATTRIBUTES : DWORD = CTL_CODE ! (IOCTL_DISK_BASE , 0x003c , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
