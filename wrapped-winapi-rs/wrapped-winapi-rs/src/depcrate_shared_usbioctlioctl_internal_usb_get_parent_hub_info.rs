// Generated macro for IOCTL_INTERNAL_USB_GET_PARENT_HUB_INFO (const)
macro_rules! Depcrate_shared_usbioctlIOCTL_INTERNAL_USB_GET_PARENT_HUB_INFO {
() => {
// Module: crate::shared::usbioctl
// Provides: {"IOCTL_INTERNAL_USB_GET_PARENT_HUB_INFO"}
// Dependencies: {}
pub const IOCTL_INTERNAL_USB_GET_PARENT_HUB_INFO : DWORD = CTL_CODE ! (FILE_DEVICE_USB , USB_GET_PARENT_HUB_INFO , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
