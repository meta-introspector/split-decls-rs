// Generated macro for IOCTL_DISK_IS_WRITABLE (const)
macro_rules! Depcrate_um_winioctlIOCTL_DISK_IS_WRITABLE {
() => {
// Module: crate::um::winioctl
// Provides: {"IOCTL_DISK_IS_WRITABLE"}
// Dependencies: {}
pub const IOCTL_DISK_IS_WRITABLE : DWORD = CTL_CODE ! (IOCTL_DISK_BASE , 0x0009 , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
