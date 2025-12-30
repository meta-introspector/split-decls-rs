// Generated macro for SMART_RCV_DRIVE_DATA (const)
macro_rules! Depcrate_um_winioctlSMART_RCV_DRIVE_DATA {
() => {
// Module: crate::um::winioctl
// Provides: {"SMART_RCV_DRIVE_DATA"}
// Dependencies: {}
pub const SMART_RCV_DRIVE_DATA : DWORD = CTL_CODE ! (IOCTL_DISK_BASE , 0x0022 , METHOD_BUFFERED , FILE_READ_ACCESS | FILE_WRITE_ACCESS) ;
};
}
