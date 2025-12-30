// Generated macro for FSCTL_TXFS_SAVEPOINT_INFORMATION (const)
macro_rules! Depcrate_um_winioctlFSCTL_TXFS_SAVEPOINT_INFORMATION {
() => {
// Module: crate::um::winioctl
// Provides: {"FSCTL_TXFS_SAVEPOINT_INFORMATION"}
// Dependencies: {}
pub const FSCTL_TXFS_SAVEPOINT_INFORMATION : DWORD = CTL_CODE ! (FILE_DEVICE_FILE_SYSTEM , 94 , METHOD_BUFFERED , FILE_WRITE_DATA) ;
};
}
