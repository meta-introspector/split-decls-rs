// Generated macro for FSCTL_DISMOUNT_VOLUME (const)
macro_rules! Depcrate_um_winioctlFSCTL_DISMOUNT_VOLUME {
() => {
// Module: crate::um::winioctl
// Provides: {"FSCTL_DISMOUNT_VOLUME"}
// Dependencies: {}
pub const FSCTL_DISMOUNT_VOLUME : DWORD = CTL_CODE ! (FILE_DEVICE_FILE_SYSTEM , 8 , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
