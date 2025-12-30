// Generated macro for FSCTL_REQUEST_BATCH_OPLOCK (const)
macro_rules! Depcrate_um_winioctlFSCTL_REQUEST_BATCH_OPLOCK {
() => {
// Module: crate::um::winioctl
// Provides: {"FSCTL_REQUEST_BATCH_OPLOCK"}
// Dependencies: {}
pub const FSCTL_REQUEST_BATCH_OPLOCK : DWORD = CTL_CODE ! (FILE_DEVICE_FILE_SYSTEM , 2 , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
