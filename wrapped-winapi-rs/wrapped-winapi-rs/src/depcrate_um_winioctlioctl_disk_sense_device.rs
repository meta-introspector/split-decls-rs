// Generated macro for IOCTL_DISK_SENSE_DEVICE (const)
macro_rules! Depcrate_um_winioctlIOCTL_DISK_SENSE_DEVICE {
() => {
// Module: crate::um::winioctl
// Provides: {"IOCTL_DISK_SENSE_DEVICE"}
// Dependencies: {}
pub const IOCTL_DISK_SENSE_DEVICE : DWORD = CTL_CODE ! (IOCTL_DISK_BASE , 0x00f8 , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
