// Generated macro for FSCTL_QUERY_ALLOCATED_RANGES (const)
macro_rules! Depcrate_um_winioctlFSCTL_QUERY_ALLOCATED_RANGES {
() => {
// Module: crate::um::winioctl
// Provides: {"FSCTL_QUERY_ALLOCATED_RANGES"}
// Dependencies: {}
pub const FSCTL_QUERY_ALLOCATED_RANGES : DWORD = CTL_CODE ! (FILE_DEVICE_FILE_SYSTEM , 51 , METHOD_NEITHER , FILE_READ_DATA) ;
};
}
