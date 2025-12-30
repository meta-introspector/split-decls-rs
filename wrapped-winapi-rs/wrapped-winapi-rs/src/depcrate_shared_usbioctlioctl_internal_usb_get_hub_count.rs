// Generated macro for IOCTL_INTERNAL_USB_GET_HUB_COUNT (const)
macro_rules! Depcrate_shared_usbioctlIOCTL_INTERNAL_USB_GET_HUB_COUNT {
() => {
// Module: crate::shared::usbioctl
// Provides: {"IOCTL_INTERNAL_USB_GET_HUB_COUNT"}
// Dependencies: {}
pub const IOCTL_INTERNAL_USB_GET_HUB_COUNT : DWORD = CTL_CODE ! (FILE_DEVICE_USB , USB_GET_HUB_COUNT , METHOD_NEITHER , FILE_ANY_ACCESS) ;
};
}
