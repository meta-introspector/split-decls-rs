// Generated macro for FSCTL_IS_PATHNAME_VALID (const)
macro_rules! Depcrate_um_winioctlFSCTL_IS_PATHNAME_VALID {
() => {
// Module: crate::um::winioctl
// Provides: {"FSCTL_IS_PATHNAME_VALID"}
// Dependencies: {}
pub const FSCTL_IS_PATHNAME_VALID : DWORD = CTL_CODE ! (FILE_DEVICE_FILE_SYSTEM , 11 , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
