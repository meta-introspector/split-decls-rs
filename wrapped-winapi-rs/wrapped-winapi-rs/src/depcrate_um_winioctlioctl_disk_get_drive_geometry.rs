// Generated macro for IOCTL_DISK_GET_DRIVE_GEOMETRY (const)
macro_rules! Depcrate_um_winioctlIOCTL_DISK_GET_DRIVE_GEOMETRY {
() => {
// Module: crate::um::winioctl
// Provides: {"IOCTL_DISK_GET_DRIVE_GEOMETRY"}
// Dependencies: {}
pub const IOCTL_DISK_GET_DRIVE_GEOMETRY : DWORD = CTL_CODE ! (IOCTL_DISK_BASE , 0x0000 , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
