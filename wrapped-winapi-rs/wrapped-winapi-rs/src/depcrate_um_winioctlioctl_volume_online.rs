// Generated macro for IOCTL_VOLUME_ONLINE (const)
macro_rules! Depcrate_um_winioctlIOCTL_VOLUME_ONLINE {
() => {
// Module: crate::um::winioctl
// Provides: {"IOCTL_VOLUME_ONLINE"}
// Dependencies: {}
pub const IOCTL_VOLUME_ONLINE : DWORD = CTL_CODE ! (IOCTL_VOLUME_BASE , 2 , METHOD_BUFFERED , FILE_READ_ACCESS | FILE_WRITE_ACCESS) ;
};
}
