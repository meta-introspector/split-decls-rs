// Generated macro for other_46 (other)
macro_rules! Depcrate_network_configurationother_46 {
() => {
// Module: crate::network_configuration
// Provides: {"other_46"}
// Dependencies: {}
extern "C" { pub fn SCNetworkCheckReachabilityByAddress (address : * const sockaddr , addrlen : socklen_t , flags : * mut SCNetworkConnectionFlags ,) -> Boolean ; pub fn SCNetworkCheckReachabilityByName (nodename : * const :: core :: ffi :: c_char , flags : * mut SCNetworkConnectionFlags ,) -> Boolean ; pub fn SCNetworkInterfaceRefreshConfiguration (ifName : CFStringRef) -> Boolean ; }
};
}
