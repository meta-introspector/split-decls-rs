// Generated macro for SMART_SEND_DRIVE_COMMAND (const)
macro_rules! Depcrate_um_winioctlSMART_SEND_DRIVE_COMMAND {
() => {
// Module: crate::um::winioctl
// Provides: {"SMART_SEND_DRIVE_COMMAND"}
// Dependencies: {}
pub const SMART_SEND_DRIVE_COMMAND : DWORD = CTL_CODE ! (IOCTL_DISK_BASE , 0x0021 , METHOD_BUFFERED , FILE_READ_ACCESS | FILE_WRITE_ACCESS) ;
};
}
