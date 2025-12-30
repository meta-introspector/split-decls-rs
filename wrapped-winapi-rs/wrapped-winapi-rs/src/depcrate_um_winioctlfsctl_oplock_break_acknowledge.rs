// Generated macro for FSCTL_OPLOCK_BREAK_ACKNOWLEDGE (const)
macro_rules! Depcrate_um_winioctlFSCTL_OPLOCK_BREAK_ACKNOWLEDGE {
() => {
// Module: crate::um::winioctl
// Provides: {"FSCTL_OPLOCK_BREAK_ACKNOWLEDGE"}
// Dependencies: {}
pub const FSCTL_OPLOCK_BREAK_ACKNOWLEDGE : DWORD = CTL_CODE ! (FILE_DEVICE_FILE_SYSTEM , 3 , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
