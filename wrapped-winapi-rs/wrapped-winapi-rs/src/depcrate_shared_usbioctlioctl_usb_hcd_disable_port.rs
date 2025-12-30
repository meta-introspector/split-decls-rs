// Generated macro for IOCTL_USB_HCD_DISABLE_PORT (const)
macro_rules! Depcrate_shared_usbioctlIOCTL_USB_HCD_DISABLE_PORT {
() => {
// Module: crate::shared::usbioctl
// Provides: {"IOCTL_USB_HCD_DISABLE_PORT"}
// Dependencies: {}
pub const IOCTL_USB_HCD_DISABLE_PORT : DWORD = CTL_CODE ! (FILE_DEVICE_USB , HCD_DISABLE_PORT , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
