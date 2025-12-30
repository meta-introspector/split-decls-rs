// Generated macro for IOCTL_INTERNAL_USB_GET_DEVICE_CONFIG_INFO (const)
macro_rules! Depcrate_shared_usbioctlIOCTL_INTERNAL_USB_GET_DEVICE_CONFIG_INFO {
() => {
// Module: crate::shared::usbioctl
// Provides: {"IOCTL_INTERNAL_USB_GET_DEVICE_CONFIG_INFO"}
// Dependencies: {}
pub const IOCTL_INTERNAL_USB_GET_DEVICE_CONFIG_INFO : DWORD = CTL_CODE ! (FILE_DEVICE_USB , USB_GET_HUB_CONFIG_INFO , METHOD_NEITHER , FILE_ANY_ACCESS) ;
};
}
