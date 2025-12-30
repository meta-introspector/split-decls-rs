// Generated macro for IOCTL_USB_DIAGNOSTIC_MODE_OFF (const)
macro_rules! Depcrate_shared_usbioctlIOCTL_USB_DIAGNOSTIC_MODE_OFF {
() => {
// Module: crate::shared::usbioctl
// Provides: {"IOCTL_USB_DIAGNOSTIC_MODE_OFF"}
// Dependencies: {}
pub const IOCTL_USB_DIAGNOSTIC_MODE_OFF : DWORD = CTL_CODE ! (FILE_DEVICE_USB , HCD_DIAGNOSTIC_MODE_OFF , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
