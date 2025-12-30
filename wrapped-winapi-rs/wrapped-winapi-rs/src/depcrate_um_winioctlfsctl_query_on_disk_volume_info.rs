// Generated macro for FSCTL_QUERY_ON_DISK_VOLUME_INFO (const)
macro_rules! Depcrate_um_winioctlFSCTL_QUERY_ON_DISK_VOLUME_INFO {
() => {
// Module: crate::um::winioctl
// Provides: {"FSCTL_QUERY_ON_DISK_VOLUME_INFO"}
// Dependencies: {}
pub const FSCTL_QUERY_ON_DISK_VOLUME_INFO : DWORD = CTL_CODE ! (FILE_DEVICE_FILE_SYSTEM , 79 , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
