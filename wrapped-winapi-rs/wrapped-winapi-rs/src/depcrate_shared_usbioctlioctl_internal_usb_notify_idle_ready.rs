// Generated macro for IOCTL_INTERNAL_USB_NOTIFY_IDLE_READY (const)
macro_rules! Depcrate_shared_usbioctlIOCTL_INTERNAL_USB_NOTIFY_IDLE_READY {
() => {
// Module: crate::shared::usbioctl
// Provides: {"IOCTL_INTERNAL_USB_NOTIFY_IDLE_READY"}
// Dependencies: {}
pub const IOCTL_INTERNAL_USB_NOTIFY_IDLE_READY : DWORD = CTL_CODE ! (FILE_DEVICE_USB , USB_IDLE_NOTIFICATION_EX , METHOD_NEITHER , FILE_ANY_ACCESS) ;
};
}
