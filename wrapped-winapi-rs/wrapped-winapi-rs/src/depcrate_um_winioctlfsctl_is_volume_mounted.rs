// Generated macro for FSCTL_IS_VOLUME_MOUNTED (const)
macro_rules! Depcrate_um_winioctlFSCTL_IS_VOLUME_MOUNTED {
() => {
// Module: crate::um::winioctl
// Provides: {"FSCTL_IS_VOLUME_MOUNTED"}
// Dependencies: {}
pub const FSCTL_IS_VOLUME_MOUNTED : DWORD = CTL_CODE ! (FILE_DEVICE_FILE_SYSTEM , 10 , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
