// Generated macro for FSCTL_SET_VOLUME_COMPRESSION_STATE (const)
macro_rules! Depcrate_um_winioctlFSCTL_SET_VOLUME_COMPRESSION_STATE {
() => {
// Module: crate::um::winioctl
// Provides: {"FSCTL_SET_VOLUME_COMPRESSION_STATE"}
// Dependencies: {}
pub const FSCTL_SET_VOLUME_COMPRESSION_STATE : DWORD = CTL_CODE ! (FILE_DEVICE_FILE_SYSTEM , 80 , METHOD_BUFFERED , FILE_SPECIAL_ACCESS) ;
};
}
