// Generated macro for FSCTL_WAIT_FOR_REPAIR (const)
macro_rules! Depcrate_um_winioctlFSCTL_WAIT_FOR_REPAIR {
() => {
// Module: crate::um::winioctl
// Provides: {"FSCTL_WAIT_FOR_REPAIR"}
// Dependencies: {}
pub const FSCTL_WAIT_FOR_REPAIR : DWORD = CTL_CODE ! (FILE_DEVICE_FILE_SYSTEM , 104 , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
