// Generated macro for IOCTL_INTERNAL_USB_SUBMIT_URB (const)
macro_rules! Depcrate_shared_usbioctlIOCTL_INTERNAL_USB_SUBMIT_URB {
() => {
// Module: crate::shared::usbioctl
// Provides: {"IOCTL_INTERNAL_USB_SUBMIT_URB"}
// Dependencies: {}
pub const IOCTL_INTERNAL_USB_SUBMIT_URB : DWORD = CTL_CODE ! (FILE_DEVICE_USB , USB_SUBMIT_URB , METHOD_NEITHER , FILE_ANY_ACCESS) ;
};
}
