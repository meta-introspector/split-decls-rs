// Generated macro for IOCTL_USB_DIAG_IGNORE_HUBS_OFF (const)
macro_rules! Depcrate_shared_usbioctlIOCTL_USB_DIAG_IGNORE_HUBS_OFF {
() => {
// Module: crate::shared::usbioctl
// Provides: {"IOCTL_USB_DIAG_IGNORE_HUBS_OFF"}
// Dependencies: {}
pub const IOCTL_USB_DIAG_IGNORE_HUBS_OFF : DWORD = CTL_CODE ! (FILE_DEVICE_USB , USB_DIAG_IGNORE_HUBS_OFF , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
