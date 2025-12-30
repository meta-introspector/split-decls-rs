// Generated macro for FSCTL_FILE_PREFETCH (const)
macro_rules! Depcrate_um_winioctlFSCTL_FILE_PREFETCH {
() => {
// Module: crate::um::winioctl
// Provides: {"FSCTL_FILE_PREFETCH"}
// Dependencies: {}
pub const FSCTL_FILE_PREFETCH : DWORD = CTL_CODE ! (FILE_DEVICE_FILE_SYSTEM , 72 , METHOD_BUFFERED , FILE_SPECIAL_ACCESS) ;
};
}
