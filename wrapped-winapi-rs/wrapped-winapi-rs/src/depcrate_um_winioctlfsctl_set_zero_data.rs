// Generated macro for FSCTL_SET_ZERO_DATA (const)
macro_rules! Depcrate_um_winioctlFSCTL_SET_ZERO_DATA {
() => {
// Module: crate::um::winioctl
// Provides: {"FSCTL_SET_ZERO_DATA"}
// Dependencies: {}
pub const FSCTL_SET_ZERO_DATA : DWORD = CTL_CODE ! (FILE_DEVICE_FILE_SYSTEM , 50 , METHOD_BUFFERED , FILE_WRITE_DATA) ;
};
}
