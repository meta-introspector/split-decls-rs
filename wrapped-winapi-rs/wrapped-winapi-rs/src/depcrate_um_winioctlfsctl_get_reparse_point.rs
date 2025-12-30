// Generated macro for FSCTL_GET_REPARSE_POINT (const)
macro_rules! Depcrate_um_winioctlFSCTL_GET_REPARSE_POINT {
() => {
// Module: crate::um::winioctl
// Provides: {"FSCTL_GET_REPARSE_POINT"}
// Dependencies: {}
pub const FSCTL_GET_REPARSE_POINT : DWORD = CTL_CODE ! (FILE_DEVICE_FILE_SYSTEM , 42 , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
