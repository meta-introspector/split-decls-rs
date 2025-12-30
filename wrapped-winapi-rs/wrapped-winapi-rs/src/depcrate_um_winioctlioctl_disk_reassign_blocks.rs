// Generated macro for IOCTL_DISK_REASSIGN_BLOCKS (const)
macro_rules! Depcrate_um_winioctlIOCTL_DISK_REASSIGN_BLOCKS {
() => {
// Module: crate::um::winioctl
// Provides: {"IOCTL_DISK_REASSIGN_BLOCKS"}
// Dependencies: {}
pub const IOCTL_DISK_REASSIGN_BLOCKS : DWORD = CTL_CODE ! (IOCTL_DISK_BASE , 0x0007 , METHOD_BUFFERED , FILE_READ_ACCESS | FILE_WRITE_ACCESS) ;
};
}
