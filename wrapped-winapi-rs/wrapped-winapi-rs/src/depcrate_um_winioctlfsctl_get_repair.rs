// Generated macro for FSCTL_GET_REPAIR (const)
macro_rules! Depcrate_um_winioctlFSCTL_GET_REPAIR {
() => {
// Module: crate::um::winioctl
// Provides: {"FSCTL_GET_REPAIR"}
// Dependencies: {}
pub const FSCTL_GET_REPAIR : DWORD = CTL_CODE ! (FILE_DEVICE_FILE_SYSTEM , 103 , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
