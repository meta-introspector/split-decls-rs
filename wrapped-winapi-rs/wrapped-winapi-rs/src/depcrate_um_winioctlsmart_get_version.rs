// Generated macro for SMART_GET_VERSION (const)
macro_rules! Depcrate_um_winioctlSMART_GET_VERSION {
() => {
// Module: crate::um::winioctl
// Provides: {"SMART_GET_VERSION"}
// Dependencies: {}
pub const SMART_GET_VERSION : DWORD = CTL_CODE ! (IOCTL_DISK_BASE , 0x0020 , METHOD_BUFFERED , FILE_READ_ACCESS) ;
};
}
