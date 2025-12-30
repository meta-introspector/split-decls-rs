// Generated macro for macro_10855 (macro)
macro_rules! Depcrate_shared_usbioctlmacro_10855 {
() => {
// Module: crate::shared::usbioctl
// Provides: {"macro_10855"}
// Dependencies: {}
STRUCT ! { # [repr (packed)] struct USB_DEVICE_NODE_INFO { Sig : ULONG , LengthInBytes : ULONG , DeviceDescription : [WCHAR ; 40] , NodeType : USB_WMI_DEVICE_NODE_TYPE , BusAddress : USB_TOPOLOGY_ADDRESS , u : USB_DEVICE_NODE_INFO_u , } }
};
}
