// Generated macro for IOCTL_STORAGE_GET_DEVICE_TELEMETRY (const)
macro_rules! Depcrate_um_winioctlIOCTL_STORAGE_GET_DEVICE_TELEMETRY {
() => {
// Module: crate::um::winioctl
// Provides: {"IOCTL_STORAGE_GET_DEVICE_TELEMETRY"}
// Dependencies: {}
pub const IOCTL_STORAGE_GET_DEVICE_TELEMETRY : DWORD = CTL_CODE ! (IOCTL_STORAGE_BASE , 0x0470 , METHOD_BUFFERED , FILE_READ_ACCESS | FILE_WRITE_ACCESS) ;
};
}
