// Generated macro for IOCTL_INTERNAL_USB_RECORD_FAILURE (const)
macro_rules! Depcrate_shared_usbioctlIOCTL_INTERNAL_USB_RECORD_FAILURE {
() => {
// Module: crate::shared::usbioctl
// Provides: {"IOCTL_INTERNAL_USB_RECORD_FAILURE"}
// Dependencies: {}
pub const IOCTL_INTERNAL_USB_RECORD_FAILURE : DWORD = CTL_CODE ! (FILE_DEVICE_USB , USB_RECORD_FAILURE , METHOD_NEITHER , FILE_ANY_ACCESS) ;
};
}
