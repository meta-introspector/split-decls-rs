// Generated macro for IOCTL_USB_GET_NODE_INFORMATION (const)
macro_rules! Depcrate_shared_usbioctlIOCTL_USB_GET_NODE_INFORMATION {
() => {
// Module: crate::shared::usbioctl
// Provides: {"IOCTL_USB_GET_NODE_INFORMATION"}
// Dependencies: {}
pub const IOCTL_USB_GET_NODE_INFORMATION : DWORD = CTL_CODE ! (FILE_DEVICE_USB , USB_GET_NODE_INFORMATION , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
