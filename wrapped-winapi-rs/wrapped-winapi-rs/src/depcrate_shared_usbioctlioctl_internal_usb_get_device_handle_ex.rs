// Generated macro for IOCTL_INTERNAL_USB_GET_DEVICE_HANDLE_EX (const)
macro_rules! Depcrate_shared_usbioctlIOCTL_INTERNAL_USB_GET_DEVICE_HANDLE_EX {
() => {
// Module: crate::shared::usbioctl
// Provides: {"IOCTL_INTERNAL_USB_GET_DEVICE_HANDLE_EX"}
// Dependencies: {}
pub const IOCTL_INTERNAL_USB_GET_DEVICE_HANDLE_EX : DWORD = CTL_CODE ! (FILE_DEVICE_USB , USB_GET_DEVICE_HANDLE_EX , METHOD_NEITHER , FILE_ANY_ACCESS) ;
};
}
