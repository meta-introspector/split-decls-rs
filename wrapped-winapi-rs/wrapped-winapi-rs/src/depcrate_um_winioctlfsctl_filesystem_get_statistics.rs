// Generated macro for FSCTL_FILESYSTEM_GET_STATISTICS (const)
macro_rules! Depcrate_um_winioctlFSCTL_FILESYSTEM_GET_STATISTICS {
() => {
// Module: crate::um::winioctl
// Provides: {"FSCTL_FILESYSTEM_GET_STATISTICS"}
// Dependencies: {}
pub const FSCTL_FILESYSTEM_GET_STATISTICS : DWORD = CTL_CODE ! (FILE_DEVICE_FILE_SYSTEM , 24 , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
