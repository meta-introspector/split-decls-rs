// Generated macro for FSCTL_GET_NTFS_FILE_RECORD (const)
macro_rules! Depcrate_um_winioctlFSCTL_GET_NTFS_FILE_RECORD {
() => {
// Module: crate::um::winioctl
// Provides: {"FSCTL_GET_NTFS_FILE_RECORD"}
// Dependencies: {}
pub const FSCTL_GET_NTFS_FILE_RECORD : DWORD = CTL_CODE ! (FILE_DEVICE_FILE_SYSTEM , 26 , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
