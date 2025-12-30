// Generated macro for IOCTL_INTERNAL_USB_GET_CONTROLLER_NAME (const)
macro_rules! Depcrate_shared_usbioctlIOCTL_INTERNAL_USB_GET_CONTROLLER_NAME {
() => {
// Module: crate::shared::usbioctl
// Provides: {"IOCTL_INTERNAL_USB_GET_CONTROLLER_NAME"}
// Dependencies: {}
pub const IOCTL_INTERNAL_USB_GET_CONTROLLER_NAME : DWORD = CTL_CODE ! (FILE_DEVICE_USB , USB_GET_CONTROLLER_NAME , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
