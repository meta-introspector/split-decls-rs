// Generated macro for USBD_PENDING (function)
macro_rules! Depcrate_shared_usbUSBD_PENDING {
() => {
// Module: crate::shared::usb
// Provides: {"USBD_PENDING"}
// Dependencies: {}
# [inline] pub fn USBD_PENDING (Status : ULONG) -> bool { (Status >> 30) == 1 }
};
}
