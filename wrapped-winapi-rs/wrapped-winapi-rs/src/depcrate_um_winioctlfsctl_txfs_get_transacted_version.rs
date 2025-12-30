// Generated macro for FSCTL_TXFS_GET_TRANSACTED_VERSION (const)
macro_rules! Depcrate_um_winioctlFSCTL_TXFS_GET_TRANSACTED_VERSION {
() => {
// Module: crate::um::winioctl
// Provides: {"FSCTL_TXFS_GET_TRANSACTED_VERSION"}
// Dependencies: {}
pub const FSCTL_TXFS_GET_TRANSACTED_VERSION : DWORD = CTL_CODE ! (FILE_DEVICE_FILE_SYSTEM , 92 , METHOD_BUFFERED , FILE_READ_DATA) ;
};
}
