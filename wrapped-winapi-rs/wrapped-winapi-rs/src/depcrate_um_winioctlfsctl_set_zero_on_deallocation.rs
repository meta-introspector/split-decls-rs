// Generated macro for FSCTL_SET_ZERO_ON_DEALLOCATION (const)
macro_rules! Depcrate_um_winioctlFSCTL_SET_ZERO_ON_DEALLOCATION {
() => {
// Module: crate::um::winioctl
// Provides: {"FSCTL_SET_ZERO_ON_DEALLOCATION"}
// Dependencies: {}
pub const FSCTL_SET_ZERO_ON_DEALLOCATION : DWORD = CTL_CODE ! (FILE_DEVICE_FILE_SYSTEM , 101 , METHOD_BUFFERED , FILE_SPECIAL_ACCESS) ;
};
}
