// Generated macro for FSCTL_LOCK_VOLUME (const)
macro_rules! Depcrate_um_winioctlFSCTL_LOCK_VOLUME {
() => {
// Module: crate::um::winioctl
// Provides: {"FSCTL_LOCK_VOLUME"}
// Dependencies: {}
pub const FSCTL_LOCK_VOLUME : DWORD = CTL_CODE ! (FILE_DEVICE_FILE_SYSTEM , 6 , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
