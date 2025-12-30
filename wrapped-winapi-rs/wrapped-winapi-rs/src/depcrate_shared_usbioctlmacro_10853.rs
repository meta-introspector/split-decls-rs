// Generated macro for macro_10853 (macro)
macro_rules! Depcrate_shared_usbioctlmacro_10853 {
() => {
// Module: crate::shared::usbioctl
// Provides: {"macro_10853"}
// Dependencies: {}
STRUCT ! { # [repr (packed)] struct USB_DEVICE_INFO { DeviceState : USB_DEVICE_STATE , PortNumber : USHORT , DeviceDescriptor : USB_DEVICE_DESCRIPTOR , CurrentConfigurationValue : UCHAR , Speed : USB_DEVICE_SPEED , DeviceAddress : USHORT , ConnectionIndex : ULONG , ConnectionStatus : USB_CONNECTION_STATUS , PnpHardwareId : [WCHAR ; 128] , PnpCompatibleId : [WCHAR ; 128] , SerialNumberId : [WCHAR ; 128] , PnpDeviceDescription : [WCHAR ; 128] , NumberOfOpenPipes : ULONG , PipeList : [USB_PIPE_INFO ; 1] , } }
};
}
