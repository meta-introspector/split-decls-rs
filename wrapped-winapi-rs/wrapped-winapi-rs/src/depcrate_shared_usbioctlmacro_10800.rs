// Generated macro for macro_10800 (macro)
macro_rules! Depcrate_shared_usbioctlmacro_10800 {
() => {
// Module: crate::shared::usbioctl
// Provides: {"macro_10800"}
// Dependencies: {}
STRUCT ! { # [repr (packed)] struct USB_NODE_CONNECTION_INFORMATION_EX { ConnectionIndex : ULONG , DeviceDescriptor : USB_DEVICE_DESCRIPTOR , CurrentConfigurationValue : UCHAR , Speed : UCHAR , DeviceIsHub : BOOLEAN , DeviceAddress : USHORT , NumberOfOpenPipes : ULONG , ConnectionStatus : USB_CONNECTION_STATUS , PipeList : [USB_PIPE_INFO ; 0] , } }
};
}
