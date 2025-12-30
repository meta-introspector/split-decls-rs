// Generated macro for macro_10780 (macro)
macro_rules! Depcrate_shared_usbioctlmacro_10780 {
() => {
// Module: crate::shared::usbioctl
// Provides: {"macro_10780"}
// Dependencies: {}
STRUCT ! { # [repr (packed)] struct USB_NODE_CONNECTION_INFORMATION { ConnectionIndex : ULONG , DeviceDescriptor : USB_DEVICE_DESCRIPTOR , CurrentConfigurationValue : UCHAR , LowSpeed : BOOLEAN , DeviceIsHub : BOOLEAN , DeviceAddress : USHORT , NumberOfOpenPipes : ULONG , ConnectionStatus : USB_CONNECTION_STATUS , PipeList : [USB_PIPE_INFO ; 0] , } }
};
}
