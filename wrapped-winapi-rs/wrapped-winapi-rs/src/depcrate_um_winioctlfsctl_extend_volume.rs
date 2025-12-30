// Generated macro for FSCTL_EXTEND_VOLUME (const)
macro_rules! Depcrate_um_winioctlFSCTL_EXTEND_VOLUME {
() => {
// Module: crate::um::winioctl
// Provides: {"FSCTL_EXTEND_VOLUME"}
// Dependencies: {}
pub const FSCTL_EXTEND_VOLUME : DWORD = CTL_CODE ! (FILE_DEVICE_FILE_SYSTEM , 60 , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
