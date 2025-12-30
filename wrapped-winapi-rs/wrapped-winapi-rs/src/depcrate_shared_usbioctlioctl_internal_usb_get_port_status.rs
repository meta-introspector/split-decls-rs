// Generated macro for IOCTL_INTERNAL_USB_GET_PORT_STATUS (const)
macro_rules! Depcrate_shared_usbioctlIOCTL_INTERNAL_USB_GET_PORT_STATUS {
() => {
// Module: crate::shared::usbioctl
// Provides: {"IOCTL_INTERNAL_USB_GET_PORT_STATUS"}
// Dependencies: {}
pub const IOCTL_INTERNAL_USB_GET_PORT_STATUS : DWORD = CTL_CODE ! (FILE_DEVICE_USB , USB_GET_PORT_STATUS , METHOD_NEITHER , FILE_ANY_ACCESS) ;
};
}
