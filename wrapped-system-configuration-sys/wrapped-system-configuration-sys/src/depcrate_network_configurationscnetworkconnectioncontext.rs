// Generated macro for SCNetworkConnectionContext (struct)
macro_rules! Depcrate_network_configurationSCNetworkConnectionContext {
() => {
// Module: crate::network_configuration
// Provides: {"SCNetworkConnectionContext"}
// Dependencies: {}
# [repr (C)] pub struct SCNetworkConnectionContext { pub version : CFIndex , pub info : * mut :: core :: ffi :: c_void , pub retain : Option < unsafe extern "C" fn (info : * const :: core :: ffi :: c_void) -> * const :: core :: ffi :: c_void , > , pub release : Option < unsafe extern "C" fn (info : * const :: core :: ffi :: c_void) > , pub copyDescription : Option < unsafe extern "C" fn (info : * const :: core :: ffi :: c_void) -> CFStringRef > , }
};
}
