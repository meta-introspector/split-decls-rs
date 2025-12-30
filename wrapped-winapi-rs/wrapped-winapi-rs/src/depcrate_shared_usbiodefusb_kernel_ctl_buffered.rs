// Generated macro for USB_KERNEL_CTL_BUFFERED (function)
macro_rules! Depcrate_shared_usbiodefUSB_KERNEL_CTL_BUFFERED {
() => {
// Module: crate::shared::usbiodef
// Provides: {"USB_KERNEL_CTL_BUFFERED"}
// Dependencies: {}
# [inline] pub fn USB_KERNEL_CTL_BUFFERED (id : ULONG) -> ULONG { CTL_CODE ! (FILE_DEVICE_USB , id , METHOD_BUFFERED , FILE_ANY_ACCESS) }
};
}
