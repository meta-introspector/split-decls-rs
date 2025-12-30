// Generated macro for FSCTL_REQUEST_OPLOCK_LEVEL_2 (const)
macro_rules! Depcrate_um_winioctlFSCTL_REQUEST_OPLOCK_LEVEL_2 {
() => {
// Module: crate::um::winioctl
// Provides: {"FSCTL_REQUEST_OPLOCK_LEVEL_2"}
// Dependencies: {}
pub const FSCTL_REQUEST_OPLOCK_LEVEL_2 : DWORD = CTL_CODE ! (FILE_DEVICE_FILE_SYSTEM , 1 , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
