// Generated macro for IOCTL_INTERNAL_USB_GET_DEVICE_HANDLE (const)
macro_rules! Depcrate_shared_usbioctlIOCTL_INTERNAL_USB_GET_DEVICE_HANDLE {
() => {
// Module: crate::shared::usbioctl
// Provides: {"IOCTL_INTERNAL_USB_GET_DEVICE_HANDLE"}
// Dependencies: {}
pub const IOCTL_INTERNAL_USB_GET_DEVICE_HANDLE : DWORD = CTL_CODE ! (FILE_DEVICE_USB , USB_GET_DEVICE_HANDLE , METHOD_NEITHER , FILE_ANY_ACCESS) ;
};
}
