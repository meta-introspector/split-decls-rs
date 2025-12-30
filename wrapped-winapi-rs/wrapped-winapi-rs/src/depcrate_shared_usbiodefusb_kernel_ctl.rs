// Generated macro for USB_KERNEL_CTL (function)
macro_rules! Depcrate_shared_usbiodefUSB_KERNEL_CTL {
() => {
// Module: crate::shared::usbiodef
// Provides: {"USB_KERNEL_CTL"}
// Dependencies: {}
# [inline] pub fn USB_KERNEL_CTL (id : ULONG) -> ULONG { CTL_CODE ! (FILE_DEVICE_USB , id , METHOD_NEITHER , FILE_ANY_ACCESS) }
};
}
