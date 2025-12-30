// Generated macro for IOCTL_GET_USB_DESCRIPTOR (const)
macro_rules! Depcrate_shared_usbscanIOCTL_GET_USB_DESCRIPTOR {
() => {
// Module: crate::shared::usbscan
// Provides: {"IOCTL_GET_USB_DESCRIPTOR"}
// Dependencies: {}
pub const IOCTL_GET_USB_DESCRIPTOR : ULONG = CTL_CODE ! (FILE_DEVICE_USB_SCAN , IOCTL_INDEX + 8 , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
