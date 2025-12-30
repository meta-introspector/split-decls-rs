// Generated macro for FSCTL_REQUEST_OPLOCK_LEVEL_1 (const)
macro_rules! Depcrate_um_winioctlFSCTL_REQUEST_OPLOCK_LEVEL_1 {
() => {
// Module: crate::um::winioctl
// Provides: {"FSCTL_REQUEST_OPLOCK_LEVEL_1"}
// Dependencies: {}
pub const FSCTL_REQUEST_OPLOCK_LEVEL_1 : DWORD = CTL_CODE ! (FILE_DEVICE_FILE_SYSTEM , 0 , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
