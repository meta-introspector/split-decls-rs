// Generated macro for IOCTL_USB_DIAG_IGNORE_HUBS_ON (const)
macro_rules! Depcrate_shared_usbioctlIOCTL_USB_DIAG_IGNORE_HUBS_ON {
() => {
// Module: crate::shared::usbioctl
// Provides: {"IOCTL_USB_DIAG_IGNORE_HUBS_ON"}
// Dependencies: {}
pub const IOCTL_USB_DIAG_IGNORE_HUBS_ON : DWORD = CTL_CODE ! (FILE_DEVICE_USB , USB_DIAG_IGNORE_HUBS_ON , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
