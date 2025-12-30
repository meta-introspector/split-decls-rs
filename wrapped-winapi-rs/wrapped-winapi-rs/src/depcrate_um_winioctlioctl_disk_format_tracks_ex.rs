// Generated macro for IOCTL_DISK_FORMAT_TRACKS_EX (const)
macro_rules! Depcrate_um_winioctlIOCTL_DISK_FORMAT_TRACKS_EX {
() => {
// Module: crate::um::winioctl
// Provides: {"IOCTL_DISK_FORMAT_TRACKS_EX"}
// Dependencies: {}
pub const IOCTL_DISK_FORMAT_TRACKS_EX : DWORD = CTL_CODE ! (IOCTL_DISK_BASE , 0x000b , METHOD_BUFFERED , FILE_READ_ACCESS | FILE_WRITE_ACCESS) ;
};
}
