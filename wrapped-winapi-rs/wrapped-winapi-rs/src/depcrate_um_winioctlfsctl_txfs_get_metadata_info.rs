// Generated macro for FSCTL_TXFS_GET_METADATA_INFO (const)
macro_rules! Depcrate_um_winioctlFSCTL_TXFS_GET_METADATA_INFO {
() => {
// Module: crate::um::winioctl
// Provides: {"FSCTL_TXFS_GET_METADATA_INFO"}
// Dependencies: {}
pub const FSCTL_TXFS_GET_METADATA_INFO : DWORD = CTL_CODE ! (FILE_DEVICE_FILE_SYSTEM , 91 , METHOD_BUFFERED , FILE_READ_DATA) ;
};
}
