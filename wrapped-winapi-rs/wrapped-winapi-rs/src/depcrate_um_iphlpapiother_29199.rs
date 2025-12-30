// Generated macro for other_29199 (other)
macro_rules! Depcrate_um_iphlpapiother_29199 {
() => {
// Module: crate::um::iphlpapi
// Provides: {"other_29199"}
// Dependencies: {}
extern "system" { pub fn GetInterfaceCurrentTimestampCapabilities (InterfaceLuid : * const NET_LUID , TimestampCapabilite : PINTERFACE_TIMESTAMP_CAPABILITIES ,) -> DWORD ; pub fn GetInterfaceHardwareTimestampCapabilities (InterfaceLuid : * const NET_LUID , TimestampCapabilite : PINTERFACE_TIMESTAMP_CAPABILITIES ,) -> DWORD ; pub fn CaptureInterfaceHardwareCrossTimestamp (InterfaceLuid : * const NET_LUID , CrossTimestamp : PINTERFACE_HARDWARE_CROSSTIMESTAMP ,) -> DWORD ; }
};
}
