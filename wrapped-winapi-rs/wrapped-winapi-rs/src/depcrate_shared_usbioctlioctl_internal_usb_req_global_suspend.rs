// Generated macro for IOCTL_INTERNAL_USB_REQ_GLOBAL_SUSPEND (const)
macro_rules! Depcrate_shared_usbioctlIOCTL_INTERNAL_USB_REQ_GLOBAL_SUSPEND {
() => {
// Module: crate::shared::usbioctl
// Provides: {"IOCTL_INTERNAL_USB_REQ_GLOBAL_SUSPEND"}
// Dependencies: {}
pub const IOCTL_INTERNAL_USB_REQ_GLOBAL_SUSPEND : DWORD = CTL_CODE ! (FILE_DEVICE_USB , USB_REQ_GLOBAL_SUSPEND , METHOD_NEITHER , FILE_ANY_ACCESS) ;
};
}
