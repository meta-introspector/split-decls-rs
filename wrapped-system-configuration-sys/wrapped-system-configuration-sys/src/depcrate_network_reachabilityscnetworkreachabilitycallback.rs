// Generated macro for SCNetworkReachabilityCallBack (type)
macro_rules! Depcrate_network_reachabilitySCNetworkReachabilityCallBack {
() => {
// Module: crate::network_reachability
// Provides: {"SCNetworkReachabilityCallBack"}
// Dependencies: {}
pub type SCNetworkReachabilityCallBack = Option < unsafe extern "C" fn (target : SCNetworkReachabilityRef , flags : SCNetworkReachabilityFlags , info : * mut :: core :: ffi :: c_void ,) , > ;
};
}
