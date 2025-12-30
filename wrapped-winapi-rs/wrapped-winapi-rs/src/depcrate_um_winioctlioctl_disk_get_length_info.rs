// Generated macro for IOCTL_DISK_GET_LENGTH_INFO (const)
macro_rules! Depcrate_um_winioctlIOCTL_DISK_GET_LENGTH_INFO {
() => {
// Module: crate::um::winioctl
// Provides: {"IOCTL_DISK_GET_LENGTH_INFO"}
// Dependencies: {}
pub const IOCTL_DISK_GET_LENGTH_INFO : DWORD = CTL_CODE ! (IOCTL_DISK_BASE , 0x0017 , METHOD_BUFFERED , FILE_READ_ACCESS) ;
};
}
