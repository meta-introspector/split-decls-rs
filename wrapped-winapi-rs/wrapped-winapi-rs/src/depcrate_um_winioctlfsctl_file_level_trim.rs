// Generated macro for FSCTL_FILE_LEVEL_TRIM (const)
macro_rules! Depcrate_um_winioctlFSCTL_FILE_LEVEL_TRIM {
() => {
// Module: crate::um::winioctl
// Provides: {"FSCTL_FILE_LEVEL_TRIM"}
// Dependencies: {}
pub const FSCTL_FILE_LEVEL_TRIM : DWORD = CTL_CODE ! (FILE_DEVICE_FILE_SYSTEM , 130 , METHOD_BUFFERED , FILE_WRITE_DATA) ;
};
}
