// Generated macro for IOCTL_GET_HCD_DRIVERKEY_NAME (const)
macro_rules! Depcrate_shared_usbioctlIOCTL_GET_HCD_DRIVERKEY_NAME {
() => {
// Module: crate::shared::usbioctl
// Provides: {"IOCTL_GET_HCD_DRIVERKEY_NAME"}
// Dependencies: {}
pub const IOCTL_GET_HCD_DRIVERKEY_NAME : DWORD = CTL_CODE ! (FILE_DEVICE_USB , HCD_GET_DRIVERKEY_NAME , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
