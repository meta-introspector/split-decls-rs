// Generated macro for FSCTL_TXFS_SHUTDOWN_RM (const)
macro_rules! Depcrate_um_winioctlFSCTL_TXFS_SHUTDOWN_RM {
() => {
// Module: crate::um::winioctl
// Provides: {"FSCTL_TXFS_SHUTDOWN_RM"}
// Dependencies: {}
pub const FSCTL_TXFS_SHUTDOWN_RM : DWORD = CTL_CODE ! (FILE_DEVICE_FILE_SYSTEM , 87 , METHOD_BUFFERED , FILE_WRITE_DATA) ;
};
}
