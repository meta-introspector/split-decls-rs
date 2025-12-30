// Generated macro for IOCTL_USB_START_TRACKING_FOR_TIME_SYNC (const)
macro_rules! Depcrate_shared_usbioctlIOCTL_USB_START_TRACKING_FOR_TIME_SYNC {
() => {
// Module: crate::shared::usbioctl
// Provides: {"IOCTL_USB_START_TRACKING_FOR_TIME_SYNC"}
// Dependencies: {}
pub const IOCTL_USB_START_TRACKING_FOR_TIME_SYNC : DWORD = CTL_CODE ! (FILE_DEVICE_USB , USB_START_TRACKING_FOR_TIME_SYNC , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
