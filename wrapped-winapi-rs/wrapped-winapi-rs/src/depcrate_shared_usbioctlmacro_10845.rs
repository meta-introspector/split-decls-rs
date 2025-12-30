// Generated macro for macro_10845 (macro)
macro_rules! Depcrate_shared_usbioctlmacro_10845 {
() => {
// Module: crate::shared::usbioctl
// Provides: {"macro_10845"}
// Dependencies: {}
STRUCT ! { # [repr (packed)] struct USB_HUB_DEVICE_INFO { HubDescriptor : USB_HUB_DESCRIPTOR , HubNumber : ULONG , DeviceAddress : USHORT , HubIsSelfPowered : BOOLEAN , HubIsRootHub : BOOLEAN , HubCapabilities : USB_HUB_CAPABILITIES , NumberOfHubPorts : ULONG , PortInfo : [USB_HUB_PORT_INFORMATION ; 1] , } }
};
}
