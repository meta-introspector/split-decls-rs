// Generated macro for FSCTL_TXFS_TRANSACTION_ACTIVE (const)
macro_rules! Depcrate_um_winioctlFSCTL_TXFS_TRANSACTION_ACTIVE {
() => {
// Module: crate::um::winioctl
// Provides: {"FSCTL_TXFS_TRANSACTION_ACTIVE"}
// Dependencies: {}
pub const FSCTL_TXFS_TRANSACTION_ACTIVE : DWORD = CTL_CODE ! (FILE_DEVICE_FILE_SYSTEM , 99 , METHOD_BUFFERED , FILE_READ_DATA) ;
};
}
