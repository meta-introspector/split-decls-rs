// Generated macro for FSCTL_SET_PERSISTENT_VOLUME_STATE (const)
macro_rules! Depcrate_um_winioctlFSCTL_SET_PERSISTENT_VOLUME_STATE {
() => {
// Module: crate::um::winioctl
// Provides: {"FSCTL_SET_PERSISTENT_VOLUME_STATE"}
// Dependencies: {}
pub const FSCTL_SET_PERSISTENT_VOLUME_STATE : DWORD = CTL_CODE ! (FILE_DEVICE_FILE_SYSTEM , 142 , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
