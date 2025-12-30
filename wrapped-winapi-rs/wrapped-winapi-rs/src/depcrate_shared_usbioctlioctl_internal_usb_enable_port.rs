// Generated macro for IOCTL_INTERNAL_USB_ENABLE_PORT (const)
macro_rules! Depcrate_shared_usbioctlIOCTL_INTERNAL_USB_ENABLE_PORT {
() => {
// Module: crate::shared::usbioctl
// Provides: {"IOCTL_INTERNAL_USB_ENABLE_PORT"}
// Dependencies: {}
pub const IOCTL_INTERNAL_USB_ENABLE_PORT : DWORD = CTL_CODE ! (FILE_DEVICE_USB , USB_ENABLE_PORT , METHOD_NEITHER , FILE_ANY_ACCESS) ;
};
}
