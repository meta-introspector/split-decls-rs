// Generated macro for FSCTL_SET_INTEGRITY_INFORMATION (const)
macro_rules! Depcrate_um_winioctlFSCTL_SET_INTEGRITY_INFORMATION {
() => {
// Module: crate::um::winioctl
// Provides: {"FSCTL_SET_INTEGRITY_INFORMATION"}
// Dependencies: {}
pub const FSCTL_SET_INTEGRITY_INFORMATION : DWORD = CTL_CODE ! (FILE_DEVICE_FILE_SYSTEM , 160 , METHOD_BUFFERED , FILE_READ_DATA | FILE_WRITE_DATA) ;
};
}
