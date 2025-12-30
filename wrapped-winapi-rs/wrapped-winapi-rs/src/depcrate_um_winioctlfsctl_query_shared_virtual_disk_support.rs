// Generated macro for FSCTL_QUERY_SHARED_VIRTUAL_DISK_SUPPORT (const)
macro_rules! Depcrate_um_winioctlFSCTL_QUERY_SHARED_VIRTUAL_DISK_SUPPORT {
() => {
// Module: crate::um::winioctl
// Provides: {"FSCTL_QUERY_SHARED_VIRTUAL_DISK_SUPPORT"}
// Dependencies: {}
pub const FSCTL_QUERY_SHARED_VIRTUAL_DISK_SUPPORT : DWORD = CTL_CODE ! (FILE_DEVICE_FILE_SYSTEM , 192 , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
