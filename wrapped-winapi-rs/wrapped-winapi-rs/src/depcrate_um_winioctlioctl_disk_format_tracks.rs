// Generated macro for IOCTL_DISK_FORMAT_TRACKS (const)
macro_rules! Depcrate_um_winioctlIOCTL_DISK_FORMAT_TRACKS {
() => {
// Module: crate::um::winioctl
// Provides: {"IOCTL_DISK_FORMAT_TRACKS"}
// Dependencies: {}
pub const IOCTL_DISK_FORMAT_TRACKS : DWORD = CTL_CODE ! (IOCTL_DISK_BASE , 0x0006 , METHOD_BUFFERED , FILE_READ_ACCESS | FILE_WRITE_ACCESS) ;
};
}
