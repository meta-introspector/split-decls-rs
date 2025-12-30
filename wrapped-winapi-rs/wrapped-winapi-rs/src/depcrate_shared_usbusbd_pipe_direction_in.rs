// Generated macro for USBD_PIPE_DIRECTION_IN (function)
macro_rules! Depcrate_shared_usbUSBD_PIPE_DIRECTION_IN {
() => {
// Module: crate::shared::usb
// Provides: {"USBD_PIPE_DIRECTION_IN"}
// Dependencies: {}
# [inline] pub fn USBD_PIPE_DIRECTION_IN (pipeInformation : & USBD_PIPE_INFORMATION) -> UCHAR { pipeInformation . EndpointAddress & USB_ENDPOINT_DIRECTION_MASK }
};
}
