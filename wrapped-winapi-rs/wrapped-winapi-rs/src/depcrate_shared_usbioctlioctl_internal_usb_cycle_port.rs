// Generated macro for IOCTL_INTERNAL_USB_CYCLE_PORT (const)
macro_rules! Depcrate_shared_usbioctlIOCTL_INTERNAL_USB_CYCLE_PORT {
() => {
// Module: crate::shared::usbioctl
// Provides: {"IOCTL_INTERNAL_USB_CYCLE_PORT"}
// Dependencies: {}
pub const IOCTL_INTERNAL_USB_CYCLE_PORT : DWORD = CTL_CODE ! (FILE_DEVICE_USB , USB_CYCLE_PORT , METHOD_NEITHER , FILE_ANY_ACCESS) ;
};
}
