// Generated macro for IOCTL_DISK_FIND_NEW_DEVICES (const)
macro_rules! Depcrate_um_winioctlIOCTL_DISK_FIND_NEW_DEVICES {
() => {
// Module: crate::um::winioctl
// Provides: {"IOCTL_DISK_FIND_NEW_DEVICES"}
// Dependencies: {}
pub const IOCTL_DISK_FIND_NEW_DEVICES : DWORD = CTL_CODE ! (IOCTL_DISK_BASE , 0x0206 , METHOD_BUFFERED , FILE_READ_ACCESS) ;
};
}
