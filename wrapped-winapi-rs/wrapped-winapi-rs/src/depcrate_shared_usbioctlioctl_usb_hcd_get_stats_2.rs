// Generated macro for IOCTL_USB_HCD_GET_STATS_2 (const)
macro_rules! Depcrate_shared_usbioctlIOCTL_USB_HCD_GET_STATS_2 {
() => {
// Module: crate::shared::usbioctl
// Provides: {"IOCTL_USB_HCD_GET_STATS_2"}
// Dependencies: {}
pub const IOCTL_USB_HCD_GET_STATS_2 : DWORD = CTL_CODE ! (FILE_DEVICE_USB , HCD_GET_STATS_2 , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
