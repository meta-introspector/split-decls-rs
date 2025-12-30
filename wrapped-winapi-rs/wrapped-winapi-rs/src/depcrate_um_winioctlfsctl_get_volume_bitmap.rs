// Generated macro for FSCTL_GET_VOLUME_BITMAP (const)
macro_rules! Depcrate_um_winioctlFSCTL_GET_VOLUME_BITMAP {
() => {
// Module: crate::um::winioctl
// Provides: {"FSCTL_GET_VOLUME_BITMAP"}
// Dependencies: {}
pub const FSCTL_GET_VOLUME_BITMAP : DWORD = CTL_CODE ! (FILE_DEVICE_FILE_SYSTEM , 27 , METHOD_NEITHER , FILE_ANY_ACCESS) ;
};
}
