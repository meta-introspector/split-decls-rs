// Generated macro for FSCTL_INVALIDATE_VOLUMES (const)
macro_rules! Depcrate_um_winioctlFSCTL_INVALIDATE_VOLUMES {
() => {
// Module: crate::um::winioctl
// Provides: {"FSCTL_INVALIDATE_VOLUMES"}
// Dependencies: {}
pub const FSCTL_INVALIDATE_VOLUMES : DWORD = CTL_CODE ! (FILE_DEVICE_FILE_SYSTEM , 21 , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
