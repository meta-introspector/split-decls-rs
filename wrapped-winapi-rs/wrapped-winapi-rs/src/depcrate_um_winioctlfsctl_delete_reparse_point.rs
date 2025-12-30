// Generated macro for FSCTL_DELETE_REPARSE_POINT (const)
macro_rules! Depcrate_um_winioctlFSCTL_DELETE_REPARSE_POINT {
() => {
// Module: crate::um::winioctl
// Provides: {"FSCTL_DELETE_REPARSE_POINT"}
// Dependencies: {}
pub const FSCTL_DELETE_REPARSE_POINT : DWORD = CTL_CODE ! (FILE_DEVICE_FILE_SYSTEM , 43 , METHOD_BUFFERED , FILE_SPECIAL_ACCESS) ;
};
}
