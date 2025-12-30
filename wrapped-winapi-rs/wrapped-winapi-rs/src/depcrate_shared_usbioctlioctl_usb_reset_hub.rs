// Generated macro for IOCTL_USB_RESET_HUB (const)
macro_rules! Depcrate_shared_usbioctlIOCTL_USB_RESET_HUB {
() => {
// Module: crate::shared::usbioctl
// Provides: {"IOCTL_USB_RESET_HUB"}
// Dependencies: {}
pub const IOCTL_USB_RESET_HUB : DWORD = CTL_CODE ! (FILE_DEVICE_USB , USB_RESET_HUB , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
