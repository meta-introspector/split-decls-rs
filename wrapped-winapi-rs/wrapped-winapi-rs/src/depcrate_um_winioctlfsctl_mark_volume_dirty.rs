// Generated macro for FSCTL_MARK_VOLUME_DIRTY (const)
macro_rules! Depcrate_um_winioctlFSCTL_MARK_VOLUME_DIRTY {
() => {
// Module: crate::um::winioctl
// Provides: {"FSCTL_MARK_VOLUME_DIRTY"}
// Dependencies: {}
pub const FSCTL_MARK_VOLUME_DIRTY : DWORD = CTL_CODE ! (FILE_DEVICE_FILE_SYSTEM , 12 , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
