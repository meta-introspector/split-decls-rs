// Generated macro for IOCTL_VOLUME_OFFLINE (const)
macro_rules! Depcrate_um_winioctlIOCTL_VOLUME_OFFLINE {
() => {
// Module: crate::um::winioctl
// Provides: {"IOCTL_VOLUME_OFFLINE"}
// Dependencies: {}
pub const IOCTL_VOLUME_OFFLINE : DWORD = CTL_CODE ! (IOCTL_VOLUME_BASE , 3 , METHOD_BUFFERED , FILE_READ_ACCESS | FILE_WRITE_ACCESS) ;
};
}
