// Generated macro for FSCTL_READ_FROM_PLEX (const)
macro_rules! Depcrate_um_winioctlFSCTL_READ_FROM_PLEX {
() => {
// Module: crate::um::winioctl
// Provides: {"FSCTL_READ_FROM_PLEX"}
// Dependencies: {}
pub const FSCTL_READ_FROM_PLEX : DWORD = CTL_CODE ! (FILE_DEVICE_FILE_SYSTEM , 71 , METHOD_OUT_DIRECT , FILE_READ_DATA) ;
};
}
