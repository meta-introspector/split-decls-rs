// Generated macro for IOCTL_USB_GET_ROOT_HUB_NAME (const)
macro_rules! Depcrate_shared_usbioctlIOCTL_USB_GET_ROOT_HUB_NAME {
() => {
// Module: crate::shared::usbioctl
// Provides: {"IOCTL_USB_GET_ROOT_HUB_NAME"}
// Dependencies: {}
pub const IOCTL_USB_GET_ROOT_HUB_NAME : DWORD = CTL_CODE ! (FILE_DEVICE_USB , HCD_GET_ROOT_HUB_NAME , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
