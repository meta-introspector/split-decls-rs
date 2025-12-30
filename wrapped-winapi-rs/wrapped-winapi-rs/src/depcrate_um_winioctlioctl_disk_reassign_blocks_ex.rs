// Generated macro for IOCTL_DISK_REASSIGN_BLOCKS_EX (const)
macro_rules! Depcrate_um_winioctlIOCTL_DISK_REASSIGN_BLOCKS_EX {
() => {
// Module: crate::um::winioctl
// Provides: {"IOCTL_DISK_REASSIGN_BLOCKS_EX"}
// Dependencies: {}
pub const IOCTL_DISK_REASSIGN_BLOCKS_EX : DWORD = CTL_CODE ! (IOCTL_DISK_BASE , 0x0029 , METHOD_BUFFERED , FILE_READ_ACCESS | FILE_WRITE_ACCESS) ;
};
}
