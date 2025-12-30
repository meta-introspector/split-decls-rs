// Generated macro for USB_ENDPOINT_DIRECTION_OUT (function)
macro_rules! Depcrate_shared_usbspecUSB_ENDPOINT_DIRECTION_OUT {
() => {
// Module: crate::shared::usbspec
// Provides: {"USB_ENDPOINT_DIRECTION_OUT"}
// Dependencies: {}
# [inline] pub fn USB_ENDPOINT_DIRECTION_OUT (addr : UCHAR) -> UCHAR { ! (addr & USB_ENDPOINT_DIRECTION_MASK) }
};
}
