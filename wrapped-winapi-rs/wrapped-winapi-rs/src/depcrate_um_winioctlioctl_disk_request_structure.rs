// Generated macro for IOCTL_DISK_REQUEST_STRUCTURE (const)
macro_rules! Depcrate_um_winioctlIOCTL_DISK_REQUEST_STRUCTURE {
() => {
// Module: crate::um::winioctl
// Provides: {"IOCTL_DISK_REQUEST_STRUCTURE"}
// Dependencies: {}
pub const IOCTL_DISK_REQUEST_STRUCTURE : DWORD = CTL_CODE ! (IOCTL_DISK_BASE , 0x000f , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
