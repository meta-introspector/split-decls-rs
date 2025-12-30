// Generated macro for FSCTL_ENABLE_UPGRADE (const)
macro_rules! Depcrate_um_winioctlFSCTL_ENABLE_UPGRADE {
() => {
// Module: crate::um::winioctl
// Provides: {"FSCTL_ENABLE_UPGRADE"}
// Dependencies: {}
pub const FSCTL_ENABLE_UPGRADE : DWORD = CTL_CODE ! (FILE_DEVICE_FILE_SYSTEM , 52 , METHOD_BUFFERED , FILE_WRITE_DATA) ;
};
}
