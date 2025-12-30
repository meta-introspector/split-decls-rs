// Generated macro for IOCTL_USB_HCD_GET_STATS_1 (const)
macro_rules! Depcrate_shared_usbioctlIOCTL_USB_HCD_GET_STATS_1 {
() => {
// Module: crate::shared::usbioctl
// Provides: {"IOCTL_USB_HCD_GET_STATS_1"}
// Dependencies: {}
pub const IOCTL_USB_HCD_GET_STATS_1 : DWORD = CTL_CODE ! (FILE_DEVICE_USB , HCD_GET_STATS_1 , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
