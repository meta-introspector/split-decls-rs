// Generated macro for FSCTL_OPLOCK_BREAK_NOTIFY (const)
macro_rules! Depcrate_um_winioctlFSCTL_OPLOCK_BREAK_NOTIFY {
() => {
// Module: crate::um::winioctl
// Provides: {"FSCTL_OPLOCK_BREAK_NOTIFY"}
// Dependencies: {}
pub const FSCTL_OPLOCK_BREAK_NOTIFY : DWORD = CTL_CODE ! (FILE_DEVICE_FILE_SYSTEM , 5 , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
