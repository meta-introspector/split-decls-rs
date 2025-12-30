// Generated macro for FSCTL_IS_VOLUME_DIRTY (const)
macro_rules! Depcrate_um_winioctlFSCTL_IS_VOLUME_DIRTY {
() => {
// Module: crate::um::winioctl
// Provides: {"FSCTL_IS_VOLUME_DIRTY"}
// Dependencies: {}
pub const FSCTL_IS_VOLUME_DIRTY : DWORD = CTL_CODE ! (FILE_DEVICE_FILE_SYSTEM , 30 , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
