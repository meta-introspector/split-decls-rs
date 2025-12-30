// Generated macro for IOCTL_DISK_GROW_PARTITION (const)
macro_rules! Depcrate_um_winioctlIOCTL_DISK_GROW_PARTITION {
() => {
// Module: crate::um::winioctl
// Provides: {"IOCTL_DISK_GROW_PARTITION"}
// Dependencies: {}
pub const IOCTL_DISK_GROW_PARTITION : DWORD = CTL_CODE ! (IOCTL_DISK_BASE , 0x0034 , METHOD_BUFFERED , FILE_READ_ACCESS | FILE_WRITE_ACCESS) ;
};
}
