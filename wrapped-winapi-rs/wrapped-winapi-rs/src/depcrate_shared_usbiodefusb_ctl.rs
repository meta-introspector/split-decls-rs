// Generated macro for USB_CTL (function)
macro_rules! Depcrate_shared_usbiodefUSB_CTL {
() => {
// Module: crate::shared::usbiodef
// Provides: {"USB_CTL"}
// Dependencies: {}
# [inline] pub fn USB_CTL (id : ULONG) -> ULONG { CTL_CODE ! (FILE_DEVICE_USB , id , METHOD_BUFFERED , FILE_ANY_ACCESS) }
};
}
