// Generated macro for FSCTL_SET_REPARSE_POINT (const)
macro_rules! Depcrate_um_winioctlFSCTL_SET_REPARSE_POINT {
() => {
// Module: crate::um::winioctl
// Provides: {"FSCTL_SET_REPARSE_POINT"}
// Dependencies: {}
pub const FSCTL_SET_REPARSE_POINT : DWORD = CTL_CODE ! (FILE_DEVICE_FILE_SYSTEM , 41 , METHOD_BUFFERED , FILE_SPECIAL_ACCESS) ;
};
}
