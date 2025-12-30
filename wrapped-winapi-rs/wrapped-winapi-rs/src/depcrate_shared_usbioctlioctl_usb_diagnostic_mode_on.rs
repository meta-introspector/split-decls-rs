// Generated macro for IOCTL_USB_DIAGNOSTIC_MODE_ON (const)
macro_rules! Depcrate_shared_usbioctlIOCTL_USB_DIAGNOSTIC_MODE_ON {
() => {
// Module: crate::shared::usbioctl
// Provides: {"IOCTL_USB_DIAGNOSTIC_MODE_ON"}
// Dependencies: {}
pub const IOCTL_USB_DIAGNOSTIC_MODE_ON : DWORD = CTL_CODE ! (FILE_DEVICE_USB , HCD_DIAGNOSTIC_MODE_ON , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
