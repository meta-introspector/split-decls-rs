// Generated macro for FSCTL_GET_NTFS_VOLUME_DATA (const)
macro_rules! Depcrate_um_winioctlFSCTL_GET_NTFS_VOLUME_DATA {
() => {
// Module: crate::um::winioctl
// Provides: {"FSCTL_GET_NTFS_VOLUME_DATA"}
// Dependencies: {}
pub const FSCTL_GET_NTFS_VOLUME_DATA : DWORD = CTL_CODE ! (FILE_DEVICE_FILE_SYSTEM , 25 , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
