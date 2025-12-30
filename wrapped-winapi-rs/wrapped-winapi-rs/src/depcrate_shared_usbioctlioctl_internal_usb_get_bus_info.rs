// Generated macro for IOCTL_INTERNAL_USB_GET_BUS_INFO (const)
macro_rules! Depcrate_shared_usbioctlIOCTL_INTERNAL_USB_GET_BUS_INFO {
() => {
// Module: crate::shared::usbioctl
// Provides: {"IOCTL_INTERNAL_USB_GET_BUS_INFO"}
// Dependencies: {}
pub const IOCTL_INTERNAL_USB_GET_BUS_INFO : DWORD = CTL_CODE ! (FILE_DEVICE_USB , USB_GET_BUS_INFO , METHOD_BUFFERED , FILE_ANY_ACCESS) ;
};
}
