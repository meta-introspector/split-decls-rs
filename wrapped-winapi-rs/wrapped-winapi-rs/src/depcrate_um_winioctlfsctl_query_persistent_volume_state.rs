// Generated macro for FSCTL_QUERY_PERSISTENT_VOLUME_STATE (const)
macro_rules! Depcrate_um_winioctlFSCTL_QUERY_PERSISTENT_VOLUME_STATE {
() => {
// Module: crate::um::winioctl
// Provides: {"FSCTL_QUERY_PERSISTENT_VOLUME_STATE"}
// Dependencies: {}
pub const FSCTL_QUERY_PERSISTENT_VOLUME_STATE : DWORD = CTL_CODE ! (FILE_DEVICE_FILE_SYSTEM , 143 , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
