// Generated macro for IOCTL_USB_GET_NODE_CONNECTION_NAME (const)
macro_rules! Depcrate_shared_usbioctlIOCTL_USB_GET_NODE_CONNECTION_NAME {
() => {
// Module: crate::shared::usbioctl
// Provides: {"IOCTL_USB_GET_NODE_CONNECTION_NAME"}
// Dependencies: {}
pub const IOCTL_USB_GET_NODE_CONNECTION_NAME : DWORD = CTL_CODE ! (FILE_DEVICE_USB , USB_GET_NODE_CONNECTION_NAME , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
