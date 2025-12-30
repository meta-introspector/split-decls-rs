// Generated macro for FSCTL_REQUEST_OPLOCK (const)
macro_rules! Depcrate_um_winioctlFSCTL_REQUEST_OPLOCK {
() => {
// Module: crate::um::winioctl
// Provides: {"FSCTL_REQUEST_OPLOCK"}
// Dependencies: {}
pub const FSCTL_REQUEST_OPLOCK : DWORD = CTL_CODE ! (FILE_DEVICE_FILE_SYSTEM , 144 , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
