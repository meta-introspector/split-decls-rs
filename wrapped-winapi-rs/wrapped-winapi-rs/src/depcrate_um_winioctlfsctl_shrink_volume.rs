// Generated macro for FSCTL_SHRINK_VOLUME (const)
macro_rules! Depcrate_um_winioctlFSCTL_SHRINK_VOLUME {
() => {
// Module: crate::um::winioctl
// Provides: {"FSCTL_SHRINK_VOLUME"}
// Dependencies: {}
pub const FSCTL_SHRINK_VOLUME : DWORD = CTL_CODE ! (FILE_DEVICE_FILE_SYSTEM , 108 , METHOD_BUFFERED , FILE_SPECIAL_ACCESS) ;
};
}
