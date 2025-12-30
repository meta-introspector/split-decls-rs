// Generated macro for IOCTL_CHANGER_QUERY_VOLUME_TAGS (const)
macro_rules! Depcrate_um_winioctlIOCTL_CHANGER_QUERY_VOLUME_TAGS {
() => {
// Module: crate::um::winioctl
// Provides: {"IOCTL_CHANGER_QUERY_VOLUME_TAGS"}
// Dependencies: {}
pub const IOCTL_CHANGER_QUERY_VOLUME_TAGS : DWORD = CTL_CODE ! (IOCTL_CHANGER_BASE , 0x000B , METHOD_BUFFERED , FILE_READ_ACCESS | FILE_WRITE_ACCESS) ;
};
}
