// Generated macro for SCNetworkReachabilityContext (struct)
macro_rules! Depcrate_network_reachabilitySCNetworkReachabilityContext {
() => {
// Module: crate::network_reachability
// Provides: {"SCNetworkReachabilityContext"}
// Dependencies: {}
# [repr (C)] pub struct SCNetworkReachabilityContext { pub version : CFIndex , pub info : * mut :: core :: ffi :: c_void , pub retain : Option < unsafe extern "C" fn (info : * const :: core :: ffi :: c_void) -> * const :: core :: ffi :: c_void , > , pub release : Option < unsafe extern "C" fn (info : * const :: core :: ffi :: c_void) > , pub copyDescription : Option < unsafe extern "C" fn (info : * const :: core :: ffi :: c_void) -> CFStringRef > , }
};
}
