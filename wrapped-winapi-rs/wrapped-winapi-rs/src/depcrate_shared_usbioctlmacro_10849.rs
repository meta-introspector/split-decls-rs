// Generated macro for macro_10849 (macro)
macro_rules! Depcrate_shared_usbioctlmacro_10849 {
() => {
// Module: crate::shared::usbioctl
// Provides: {"macro_10849"}
// Dependencies: {}
STRUCT ! { # [repr (packed)] struct USB_COMPOSITE_DEVICE_INFO { DeviceDescriptor : USB_DEVICE_DESCRIPTOR , CurrentConfigDescriptor : USB_CONFIGURATION_DESCRIPTOR , CurrentConfigurationValue : UCHAR , NumberOfFunctions : UCHAR , FunctionInfo : [USB_COMPOSITE_FUNCTION_INFO ; 1] , } }
};
}
