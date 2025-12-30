// Generated macro for FSCTL_SET_SPARSE (const)
macro_rules! Depcrate_um_winioctlFSCTL_SET_SPARSE {
() => {
// Module: crate::um::winioctl
// Provides: {"FSCTL_SET_SPARSE"}
// Dependencies: {}
pub const FSCTL_SET_SPARSE : DWORD = CTL_CODE ! (FILE_DEVICE_FILE_SYSTEM , 49 , METHOD_BUFFERED , FILE_SPECIAL_ACCESS) ;
};
}
