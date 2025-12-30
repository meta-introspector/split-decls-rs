// Generated macro for IOCTL_USB_GET_HUB_INFORMATION_EX (const)
macro_rules! Depcrate_shared_usbioctlIOCTL_USB_GET_HUB_INFORMATION_EX {
() => {
// Module: crate::shared::usbioctl
// Provides: {"IOCTL_USB_GET_HUB_INFORMATION_EX"}
// Dependencies: {}
pub const IOCTL_USB_GET_HUB_INFORMATION_EX : DWORD = CTL_CODE ! (FILE_DEVICE_USB , USB_GET_HUB_INFORMATION_EX , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
