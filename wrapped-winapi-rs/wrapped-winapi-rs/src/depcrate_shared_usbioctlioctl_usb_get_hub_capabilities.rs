// Generated macro for IOCTL_USB_GET_HUB_CAPABILITIES (const)
macro_rules! Depcrate_shared_usbioctlIOCTL_USB_GET_HUB_CAPABILITIES {
() => {
// Module: crate::shared::usbioctl
// Provides: {"IOCTL_USB_GET_HUB_CAPABILITIES"}
// Dependencies: {}
pub const IOCTL_USB_GET_HUB_CAPABILITIES : DWORD = CTL_CODE ! (FILE_DEVICE_USB , USB_GET_HUB_CAPABILITIES , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
