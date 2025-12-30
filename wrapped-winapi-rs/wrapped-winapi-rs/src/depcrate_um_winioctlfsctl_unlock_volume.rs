// Generated macro for FSCTL_UNLOCK_VOLUME (const)
macro_rules! Depcrate_um_winioctlFSCTL_UNLOCK_VOLUME {
() => {
// Module: crate::um::winioctl
// Provides: {"FSCTL_UNLOCK_VOLUME"}
// Dependencies: {}
pub const FSCTL_UNLOCK_VOLUME : DWORD = CTL_CODE ! (FILE_DEVICE_FILE_SYSTEM , 7 , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
