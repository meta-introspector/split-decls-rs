// Generated macro for IOCTL_STORAGE_READ_CAPACITY (const)
macro_rules! Depcrate_um_winioctlIOCTL_STORAGE_READ_CAPACITY {
() => {
// Module: crate::um::winioctl
// Provides: {"IOCTL_STORAGE_READ_CAPACITY"}
// Dependencies: {}
pub const IOCTL_STORAGE_READ_CAPACITY : DWORD = CTL_CODE ! (IOCTL_STORAGE_BASE , 0x0450 , METHOD_BUFFERED , FILE_READ_ACCESS) ;
};
}
