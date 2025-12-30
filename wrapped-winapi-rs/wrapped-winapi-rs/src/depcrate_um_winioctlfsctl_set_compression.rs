// Generated macro for FSCTL_SET_COMPRESSION (const)
macro_rules! Depcrate_um_winioctlFSCTL_SET_COMPRESSION {
() => {
// Module: crate::um::winioctl
// Provides: {"FSCTL_SET_COMPRESSION"}
// Dependencies: {}
pub const FSCTL_SET_COMPRESSION : DWORD = CTL_CODE ! (FILE_DEVICE_FILE_SYSTEM , 16 , METHOD_BUFFERED , FILE_READ_DATA | FILE_WRITE_DATA) ;
};
}
