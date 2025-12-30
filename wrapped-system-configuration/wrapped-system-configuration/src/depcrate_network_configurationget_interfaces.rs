// Generated macro for get_interfaces (function)
macro_rules! Depcrate_network_configurationget_interfaces {
() => {
// Module: crate::network_configuration
// Provides: {"get_interfaces"}
// Dependencies: {}
# [doc = " Retrieve all current network interfaces"] # [doc = ""] # [doc = " See [`SCNetworkInterfaceCopyAll`] for more details."] # [doc = ""] # [doc = " [`SCNetworkInterfaceCopyAll`]: https://developer.apple.com/documentation/systemconfiguration/1517090-scnetworkinterfacecopyall?language=objc"] pub fn get_interfaces () -> CFArray < SCNetworkInterface > { unsafe { CFArray :: < SCNetworkInterface > :: wrap_under_create_rule (SCNetworkInterfaceCopyAll ()) } }
};
}
