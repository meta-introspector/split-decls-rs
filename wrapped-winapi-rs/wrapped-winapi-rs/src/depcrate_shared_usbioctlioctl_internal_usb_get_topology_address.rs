// Generated macro for IOCTL_INTERNAL_USB_GET_TOPOLOGY_ADDRESS (const)
macro_rules! Depcrate_shared_usbioctlIOCTL_INTERNAL_USB_GET_TOPOLOGY_ADDRESS {
() => {
// Module: crate::shared::usbioctl
// Provides: {"IOCTL_INTERNAL_USB_GET_TOPOLOGY_ADDRESS"}
// Dependencies: {}
pub const IOCTL_INTERNAL_USB_GET_TOPOLOGY_ADDRESS : DWORD = CTL_CODE ! (FILE_DEVICE_USB , USB_GET_TOPOLOGY_ADDRESS , METHOD_NEITHER , FILE_ANY_ACCESS) ;
};
}
