// Generated macro for macro_10813 (macro)
macro_rules! Depcrate_shared_usbioctlmacro_10813 {
() => {
// Module: crate::shared::usbioctl
// Provides: {"macro_10813"}
// Dependencies: {}
STRUCT ! { # [repr (packed)] struct HUB_DEVICE_CONFIG_INFO { Version : ULONG , Length : ULONG , HubFlags : USB_HUB_CAP_FLAGS , HardwareIds : USB_ID_STRING , CompatibleIds : USB_ID_STRING , DeviceDescription : USB_ID_STRING , Reserved : [ULONG ; 19] , UxdSettings : USB_HUB_DEVICE_UXD_SETTINGS , } }
};
}
