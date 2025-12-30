// Generated macro for IOCTL_INTERNAL_USB_GET_ROOTHUB_PDO (const)
macro_rules! Depcrate_shared_usbioctlIOCTL_INTERNAL_USB_GET_ROOTHUB_PDO {
() => {
// Module: crate::shared::usbioctl
// Provides: {"IOCTL_INTERNAL_USB_GET_ROOTHUB_PDO"}
// Dependencies: {}
pub const IOCTL_INTERNAL_USB_GET_ROOTHUB_PDO : DWORD = CTL_CODE ! (FILE_DEVICE_USB , USB_GET_ROOTHUB_PDO , METHOD_NEITHER , FILE_ANY_ACCESS) ;
};
}
