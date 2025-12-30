// Generated macro for IOCTL_INTERNAL_USB_REQ_GLOBAL_RESUME (const)
macro_rules! Depcrate_shared_usbioctlIOCTL_INTERNAL_USB_REQ_GLOBAL_RESUME {
() => {
// Module: crate::shared::usbioctl
// Provides: {"IOCTL_INTERNAL_USB_REQ_GLOBAL_RESUME"}
// Dependencies: {}
pub const IOCTL_INTERNAL_USB_REQ_GLOBAL_RESUME : DWORD = CTL_CODE ! (FILE_DEVICE_USB , USB_REQ_GLOBAL_RESUME , METHOD_NEITHER , FILE_ANY_ACCESS) ;
};
}
