// Generated macro for FSCTL_REPAIR_COPIES (const)
macro_rules! Depcrate_um_winioctlFSCTL_REPAIR_COPIES {
() => {
// Module: crate::um::winioctl
// Provides: {"FSCTL_REPAIR_COPIES"}
// Dependencies: {}
pub const FSCTL_REPAIR_COPIES : DWORD = CTL_CODE ! (FILE_DEVICE_FILE_SYSTEM , 173 , METHOD_BUFFERED , FILE_READ_DATA | FILE_WRITE_DATA) ;
};
}
